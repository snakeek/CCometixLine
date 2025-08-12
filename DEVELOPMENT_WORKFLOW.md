# 开发工作流程

## 🔄 日常开发流程

### 1. 开发环境设置
```bash
# 安装开发工具
cargo install cargo-watch cargo-expand

# 启动自动重新编译
cargo watch -x check -x test -x run

# 代码格式化和检查
cargo fmt
cargo clippy
```

### 2. 添加新功能的标准流程

#### Step 1: 创建新段
```bash
# 创建新的段文件
touch src/core/segments/your_segment.rs
```

#### Step 2: 实现 Segment trait
```rust
// 在新文件中实现必要的结构和方法
impl Segment for YourSegment {
    fn render(&self, input: &InputData) -> String { ... }
    fn enabled(&self) -> bool { ... }
}
```

#### Step 3: 更新模块导出
```rust
// 在 src/core/segments/mod.rs 中添加
pub mod your_segment;
pub use your_segment::YourSegment;
```

#### Step 4: 更新配置
```rust
// 在 src/config/types.rs 中添加配置项
pub struct SegmentsConfig {
    // ... 其他字段
    pub your_segment: bool,
}
```

#### Step 5: 集成到状态栏
```rust
// 在 src/core/statusline.rs 中添加渲染逻辑
if self.config.segments.your_segment {
    let segment = YourSegment::new(true);
    let content = segment.render(input);
    segments.push(format!("\x1b[1;32m{}\x1b[0m", content));
}
```

#### Step 6: 编写测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_segment() {
        // 测试逻辑
    }
}
```

### 3. 测试策略

#### 单元测试
```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test segments::your_segment

# 运行测试并显示输出
cargo test -- --nocapture
```

#### 集成测试
```bash
# 创建测试输入数据
echo '{"model":{"display_name":"claude-3-5-sonnet"},"workspace":{"current_dir":"/test"},"transcript_path":"/tmp/test.jsonl"}' | cargo run
```

### 4. 性能优化技巧

#### 避免重复计算
```rust
// 使用 lazy_static 或 once_cell 缓存昂贵的计算
use std::sync::OnceLock;

static EXPENSIVE_COMPUTATION: OnceLock<String> = OnceLock::new();
```

#### 减少系统调用
```rust
// 批量处理 Git 命令
let output = Command::new("git")
    .args(["status", "--porcelain", "--branch"])
    .output()?;
```

### 5. 调试技巧

#### 使用环境变量控制日志
```rust
// 在代码中添加调试输出
#[cfg(debug_assertions)]
eprintln!("Debug: {}", value);
```

#### 使用 cargo expand 查看宏展开
```bash
cargo expand --bin ccometixline
```

## 🎨 自定义主题开发

### 颜色代码参考
```rust
// ANSI 颜色代码
"\x1b[1;31m"  // 亮红色
"\x1b[1;32m"  // 亮绿色  
"\x1b[1;33m"  // 亮黄色
"\x1b[1;34m"  // 亮蓝色
"\x1b[1;35m"  // 亮紫色
"\x1b[1;36m"  // 亮青色
"\x1b[0m"     // 重置颜色
```

### 主题配置结构
```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub model_color: String,
    pub directory_color: String,
    pub git_color: String,
    pub usage_color: String,
    pub separator_color: String,
}
```

## 📦 发布流程

### 1. 版本更新
```bash
# 更新 Cargo.toml 中的版本号
# 更新 CHANGELOG.md
# 提交更改
git add .
git commit -m "chore: bump version to x.y.z"
git tag vx.y.z
```

### 2. 构建发布版本
```bash
# 构建优化版本
cargo build --release

# 运行完整测试套件
cargo test --release
```

### 3. 跨平台构建
```bash
# 添加目标平台
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# 构建不同平台版本
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

## 🐛 常见问题解决

### 编译错误
- 检查 Rust 版本: `rustc --version`
- 更新依赖: `cargo update`
- 清理构建缓存: `cargo clean`

### 运行时错误
- 检查输入数据格式
- 验证 Git 仓库状态
- 确认文件权限

### 性能问题
- 使用 `cargo flamegraph` 进行性能分析
- 检查不必要的字符串分配
- 优化系统调用频率