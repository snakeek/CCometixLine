# CCometixLine 二次开发指南

## 项目架构理解

### 核心设计模式
- **Trait-based 设计**: 所有段都实现 `Segment` trait，便于扩展
- **配置驱动**: 通过配置控制功能开关
- **模块化**: 每个功能独立成模块，低耦合

### 关键文件说明
- `src/main.rs`: 程序入口，处理 CLI 和数据流
- `src/core/statusline.rs`: 状态栏生成器，组装各个段
- `src/core/segments/`: 各种段的实现
- `src/config/`: 配置系统

## 常见二次开发场景

### 1. 添加新的状态栏段
创建新文件 `src/core/segments/your_segment.rs`:

```rust
use super::Segment;
use crate::config::InputData;

pub struct YourSegment {
    enabled: bool,
}

impl YourSegment {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Segment for YourSegment {
    fn render(&self, input: &InputData) -> String {
        if !self.enabled {
            return String::new();
        }
        
        // 你的逻辑
        format!("🔥 Your Content")
    }
    
    fn enabled(&self) -> bool {
        self.enabled
    }
}
```

### 2. 修改配置结构
在 `src/config/types.rs` 中添加新配置项：

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SegmentsConfig {
    pub directory: bool,
    pub git: bool,
    pub model: bool,
    pub usage: bool,
    pub your_segment: bool,  // 新增
}
```

### 3. 集成新段到状态栏
在 `src/core/statusline.rs` 中添加：

```rust
if self.config.segments.your_segment {
    let your_segment = YourSegment::new(true);
    let content = your_segment.render(input);
    segments.push(format!("\x1b[1;31m{}\x1b[0m", content));
}
```

## 开发技巧

### 调试技巧
```bash
# 使用 cargo watch 自动重新编译
cargo install cargo-watch
cargo watch -x 'build'

# 调试输出
RUST_LOG=debug cargo run

# 测试特定模块
cargo test segments::git
```

### 性能优化
- 避免不必要的系统调用
- 缓存计算结果
- 使用 `std::process::Command` 时注意错误处理

### 代码风格
- 遵循 Rust 官方风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量