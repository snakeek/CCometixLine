use super::Segment;
use crate::config::{InputData, TranscriptEntry};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CONTEXT_LIMIT: u32 = 200000;

pub struct UsageSegment {
    enabled: bool,
}

impl UsageSegment {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    // 从模型名称中提取基础名称
    fn extract_model_name(model_display: &str) -> &'static str {
        if model_display.contains("claude-3-5-sonnet") {
            "claude-3-5-sonnet-20241022"
        } else if model_display.contains("haiku") {
            "claude-3-haiku-20240307"
        } else if model_display.contains("gpt-4") {
            "gpt-4"
        } else if model_display.contains("gpt-3.5") {
            "gpt-3.5-turbo"
        } else if model_display.contains("glm-4.5") {
            "glm-4.5"
        } else if model_display.contains("glm-4") {
            "glm-4"
        } else {
            // 默认使用claude-3-5-sonnet的统计方式
            "claude-3-5-sonnet-20241022"
        }
    }
    
    // 根据模型类型计算token使用量
    fn calculate_tokens_for_model(&self, usage: &crate::config::Usage, model_name: &str) -> u32 {
        match model_name {
            "glm-4.5" | "glm-4" => {
                // GLM模型优先使用total_tokens，如果没有则计算input_tokens + output_tokens
                if let Some(total) = usage.total_tokens {
                    if total > 0 {
                        return total;
                    }
                }
                usage.input_tokens.unwrap_or(0) + usage.output_tokens.unwrap_or(0)
            },
            "claude-3-5-sonnet-20241022" | "claude-3-haiku-20240307" => {
                // Claude模型使用输入token总和（包含缓存）
                usage.input_tokens.unwrap_or(0)
                    + usage.cache_creation_input_tokens.unwrap_or(0)
                    + usage.cache_read_input_tokens.unwrap_or(0)
            },
            "gpt-4" | "gpt-3.5-turbo" => {
                // GPT模型使用输入+输出token
                usage.input_tokens.unwrap_or(0) + usage.output_tokens.unwrap_or(0)
            },
            _ => {
                // 默认使用Claude的统计方式
                usage.input_tokens.unwrap_or(0)
                    + usage.cache_creation_input_tokens.unwrap_or(0)
                    + usage.cache_read_input_tokens.unwrap_or(0)
            }
        }
    }
}

impl Segment for UsageSegment {
    fn render(&self, input: &InputData) -> String {
        if !self.enabled {
            return String::new();
        }

        let context_used_token = parse_transcript_usage(&input.transcript_path);
        let context_used_rate = (context_used_token as f64 / CONTEXT_LIMIT as f64) * 100.0;

        // Format percentage: show integer when whole number, decimal when fractional
        let percentage_display = if context_used_rate.fract() == 0.0 {
            format!("{:.0}%", context_used_rate)
        } else {
            format!("{:.1}%", context_used_rate)
        };

        // Format tokens: show integer k when whole number, decimal k when fractional
        let tokens_display = if context_used_token >= 1000 {
            let k_value = context_used_token as f64 / 1000.0;
            if k_value.fract() == 0.0 {
                format!("{}k", k_value as u32)
            } else {
                format!("{:.1}k", k_value)
            }
        } else {
            context_used_token.to_string()
        };

        format!(
            "\u{f49b} {} · {} tokens",
            percentage_display, tokens_display
        )
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}

fn parse_transcript_usage<P: AsRef<Path>>(transcript_path: P) -> u32 {
    let file = match fs::File::open(&transcript_path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
            if entry.entry_type == "assistant" {
                if let Some(message) = &entry.message {
                    if let Some(usage) = &message.usage {
                        return calculate_input_tokens(usage);
                    }
                }
            }
        }
    }

    0
}

fn calculate_input_tokens(usage: &crate::config::Usage) -> u32 {
    // Priority 1: total_tokens (most accurate, includes all costs)
    if let Some(total_tokens) = usage.total_tokens {
        return total_tokens;
    }

    // Priority 2: Claude complete format (backward compatibility)
    let claude_input = usage.input_tokens.unwrap_or(0)
        + usage.cache_creation_input_tokens.unwrap_or(0)
        + usage.cache_read_input_tokens.unwrap_or(0);

    if claude_input > 0 {
        return claude_input;
    }

    // Priority 3: OpenAI manual calculation (fallback)
    if let Some(prompt_tokens) = usage.prompt_tokens {
        let completion_tokens = usage.completion_tokens.or(usage.output_tokens).unwrap_or(0);
        return prompt_tokens + completion_tokens;
    }

    // Priority 4: Input tokens only (last resort)
    usage.input_tokens.or(usage.prompt_tokens).unwrap_or(0)
}
