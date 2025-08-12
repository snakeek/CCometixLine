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
                if usage.total_tokens > 0 {
                    usage.total_tokens
                } else {
                    usage.input_tokens + usage.output_tokens
                }
            },
            "claude-3-5-sonnet-20241022" | "claude-3-haiku-20240307" => {
                // Claude模型使用输入token总和（包含缓存）
                usage.input_tokens 
                    + usage.cache_creation_input_tokens 
                    + usage.cache_read_input_tokens
            },
            "gpt-4" | "gpt-3.5-turbo" => {
                // GPT模型使用输入+输出token
                usage.input_tokens + usage.output_tokens
            },
            _ => {
                // 默认使用Claude的统计方式
                usage.input_tokens 
                    + usage.cache_creation_input_tokens 
                    + usage.cache_read_input_tokens
            }
        }
    }
}

impl Segment for UsageSegment {
    fn render(&self, input: &InputData) -> String {
        if !self.enabled {
            return String::new();
        }
        
        let model_name = Self::extract_model_name(&input.model.display_name);
        let context_used_token = parse_transcript_usage_with_model(&input.transcript_path, model_name);
        let context_used_rate = (context_used_token as f64 / CONTEXT_LIMIT as f64) * 100.0;
        let tokens_display = if context_used_token >= 1000 {
            format!("{:.1}k", context_used_token as f64 / 1000.0)
        } else {
            context_used_token.to_string()
        };
        
        format!(
            "\u{f49b} {:.1}% · {} tokens",
            context_used_rate, tokens_display
        )
    }
    
    fn enabled(&self) -> bool {
        self.enabled
    }
}

fn parse_transcript_usage_with_model<P: AsRef<Path>>(transcript_path: P, model_name: &str) -> u32 {
    let file = match fs::File::open(&transcript_path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>().unwrap_or_default();
    
    let mut total_tokens = 0;

    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
            if entry.entry_type == "assistant" {
                if let Some(message) = &entry.message {
                    if let Some(usage) = &message.usage {
                        let usage_segment = UsageSegment::new(true);
                        total_tokens += usage_segment.calculate_tokens_for_model(usage, model_name);
                    }
                }
            }
        }
    }

    total_tokens
}