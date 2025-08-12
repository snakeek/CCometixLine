use super::Segment;
use crate::config::{InputData, TranscriptEntry};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

// 费用计算配置 (价格每1M tokens)
fn get_pricing() -> HashMap<&'static str, (f64, f64)> {
    [
        ("claude-3-5-sonnet-20241022", (3.00, 15.00)),   // 输入$3/1M, 输出$15/1M
        ("claude-3-haiku-20240307", (0.25, 1.25)),        // 输入$0.25/1M, 输出$1.25/1M
        ("gpt-4", (30.00, 60.00)),                       // 输入$30/1M, 输出$60/1M
        ("gpt-3.5-turbo", (0.50, 1.50)),                 // 输入$0.50/1M, 输出$1.50/1M
        ("glm-4", (5.00, 25.00)),                        // 智谱清言GLM-4示例价格
        ("glm-4.5", (4.00, 16.00)),                      // 智谱清言GLM-4.5价格：输入4元/1M, 输出16元/1M
    ].iter().cloned().collect()
}

pub struct CostSegment {
    enabled: bool,
}

impl CostSegment {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    // 从模型名称中提取基础名称用于定价查找
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
            // 默认使用claude-3-5-sonnet的价格
            "claude-3-5-sonnet-20241022"
        }
    }
    
    // 计算总费用
    fn calculate_total_cost(&self, input_tokens: u32, output_tokens: u32, model_name: &str) -> f64 {
        let pricing = get_pricing();
        if let Some((input_price, output_price)) = pricing.get(model_name) {
            let input_cost = (input_tokens as f64 / 1_000_000.0) * input_price;
            let output_cost = (output_tokens as f64 / 1_000_000.0) * output_price;
            input_cost + output_cost
        } else {
            0.0
        }
    }
    
    // 解析transcript文件获取完整的token使用情况（根据模型类型）
    fn parse_transcript_costs<P: AsRef<Path>>(&self, transcript_path: P, model_name: &str) -> (u32, u32) {
        let file = match fs::File::open(&transcript_path) {
            Ok(file) => file,
            Err(_) => return (0, 0),
        };

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>().unwrap_or_default();
        
        let mut total_input_tokens = 0;
        let mut total_output_tokens = 0;

        for line in lines.iter().rev() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
                if entry.entry_type == "assistant" {
                    if let Some(message) = &entry.message {
                        if let Some(usage) = &message.usage {
                            match model_name {
                                "glm-4.5" | "glm-4" => {
                                    // GLM模型：如果total_tokens存在，估算输入输出比例
                                    if let Some(total) = usage.total_tokens {
                                        if total > 0 {
                                            // 简单估算：假设70%输入，30%输出
                                            let estimated_input = (total as f64 * 0.7) as u32;
                                            let estimated_output = total - estimated_input;
                                            total_input_tokens += estimated_input;
                                            total_output_tokens += estimated_output;
                                        } else {
                                            total_input_tokens += usage.input_tokens.unwrap_or(0);
                                            total_output_tokens += usage.output_tokens.unwrap_or(0);
                                        }
                                    } else {
                                        total_input_tokens += usage.input_tokens.unwrap_or(0);
                                        total_output_tokens += usage.output_tokens.unwrap_or(0);
                                    }
                                },
                                "claude-3-5-sonnet-20241022" | "claude-3-haiku-20240307" => {
                                    // Claude模型使用标准字段
                                    total_input_tokens += usage.input_tokens.unwrap_or(0)
                                        + usage.cache_creation_input_tokens.unwrap_or(0)
                                        + usage.cache_read_input_tokens.unwrap_or(0);
                                    total_output_tokens += usage.output_tokens.unwrap_or(0);
                                },
                                "gpt-4" | "gpt-3.5-turbo" => {
                                    // GPT模型使用标准字段
                                    total_input_tokens += usage.input_tokens.unwrap_or(0);
                                    total_output_tokens += usage.output_tokens.unwrap_or(0);
                                },
                                _ => {
                                    // 默认使用Claude的统计方式
                                    total_input_tokens += usage.input_tokens.unwrap_or(0)
                                        + usage.cache_creation_input_tokens.unwrap_or(0)
                                        + usage.cache_read_input_tokens.unwrap_or(0);
                                    total_output_tokens += usage.output_tokens.unwrap_or(0);
                                }
                            }
                        }
                    }
                }
            }
        }

        (total_input_tokens, total_output_tokens)
    }
}

impl Segment for CostSegment {
    fn render(&self, input: &InputData) -> String {
        if !self.enabled {
            return String::new();
        }
        
        let model_name = Self::extract_model_name(&input.model.display_name);
        let (input_tokens, output_tokens) = self.parse_transcript_costs(&input.transcript_path, model_name);
        
        if input_tokens == 0 && output_tokens == 0 {
            return String::from("\u{f09d3} $0.000");
        }
        
        let total_cost = self.calculate_total_cost(input_tokens, output_tokens, model_name);
        
        // 根据费用大小选择显示单位
        let cost_display = if total_cost < 0.01 {
            format!("${:.4}", total_cost)
        } else if total_cost < 1.0 {
            format!("${:.3}", total_cost)
        } else {
            format!("${:.2}", total_cost)
        };
        
        format!("\u{f09d3} {}", cost_display)
    }
    
    fn enabled(&self) -> bool {
        self.enabled
    }
}