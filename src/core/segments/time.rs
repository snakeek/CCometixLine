use super::Segment;
use crate::config::InputData;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimeSegment {
    enabled: bool,
    format: TimeFormat,
}

#[derive(Debug, Clone)]
pub enum TimeFormat {
    HourMinute,    // 14:30
    HourMinuteSecond, // 14:30:45
    Timestamp,     // Unix timestamp
}

impl TimeSegment {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            format: TimeFormat::HourMinute,
        }
    }
    
    pub fn with_format(mut self, format: TimeFormat) -> Self {
        self.format = format;
        self
    }
}

impl Segment for TimeSegment {
    fn render(&self, _input: &InputData) -> String {
        if !self.enabled {
            return String::new();
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        match self.format {
            TimeFormat::HourMinute => {
                let hours = (now / 3600) % 24;
                let minutes = (now / 60) % 60;
                format!("🕐 {:02}:{:02}", hours, minutes)
            }
            TimeFormat::HourMinuteSecond => {
                let hours = (now / 3600) % 24;
                let minutes = (now / 60) % 60;
                let seconds = now % 60;
                format!("🕐 {:02}:{:02}:{:02}", hours, minutes, seconds)
            }
            TimeFormat::Timestamp => {
                format!("🕐 {}", now)
            }
        }
    }
    
    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{InputData, Model, Workspace};
    
    #[test]
    fn test_time_segment_disabled() {
        let segment = TimeSegment::new(false);
        let input = create_test_input();
        assert_eq!(segment.render(&input), "");
    }
    
    #[test]
    fn test_time_segment_enabled() {
        let segment = TimeSegment::new(true);
        let input = create_test_input();
        let result = segment.render(&input);
        assert!(result.starts_with("🕐"));
        assert!(result.contains(":"));
    }
    
    fn create_test_input() -> InputData {
        InputData {
            model: Model {
                display_name: "test-model".to_string(),
            },
            workspace: Workspace {
                current_dir: "/test".to_string(),
            },
            transcript_path: "/test/transcript.jsonl".to_string(),
        }
    }
}