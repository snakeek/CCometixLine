# CCometixLine 使用指南

## 🎯 快速开始

### 1. 构建和安装

```bash
# 克隆项目
git clone <your-repo-url>
cd CCometixLine

# 构建发布版本
cargo build --release

# 安装到系统
mkdir -p ~/.claude/ccline
cp target/release/ccometixline ~/.claude/ccline/ccline
chmod +x ~/.claude/ccline/ccline
```

### 2. Claude Code 配置

在 Claude Code 的 `settings.json` 中添加：

**macOS/Linux:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "~/.claude/ccline/ccline",
    "padding": 0
  }
}
```

**Windows:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "%USERPROFILE%\\.claude\\ccline\\ccline.exe",
    "padding": 0
  }
}
```

### 3. 独立测试

如果你想在没有 Claude Code 的情况下测试，可以创建测试数据：

```bash
# 创建测试输入
cat > test_input.json << EOF
{
  "model": {
    "display_name": "claude-3-5-sonnet-20241022"
  },
  "workspace": {
    "current_dir": "$(pwd)"
  },
  "transcript_path": "test_transcript.jsonl"
}
EOF

# 创建测试转录文件
cat > test_transcript.jsonl << 'EOF'
{"type": "user", "message": {"content": "Hello"}}
{"type": "assistant", "message": {"content": "Hi!", "usage": {"input_tokens": 1500, "cache_creation_input_tokens": 500, "cache_read_input_tokens": 200}}}
EOF

# 测试运行
cat test_input.json | ~/.claude/ccline/ccline
```

## 📊 状态栏解读

### 显示格式
```
🤖 模型名 | 📁 目录名 | 🌿 Git状态 | 📊 使用情况
```

### 各部分说明

#### 1. 模型段 (🤖)
- 显示当前使用的 Claude 模型
- 自动简化模型名称：
  - `claude-3-5-sonnet` → `Sonnet 3.5`
  - `claude-3-haiku` → `Haiku 3`
  - `claude-4-opus` → `Opus 4`

#### 2. 目录段 (📁)
- 显示当前工作目录的名称
- 只显示最后一级目录名

#### 3. Git 段 (🌿)
- **分支名**: 当前 Git 分支
- **状态指示器**:
  - `✓` 干净的工作区
  - `●` 有未提交的更改
  - `⚠` 有合并冲突
- **远程跟踪**:
  - `↑n` 领先远程 n 个提交
  - `↓n` 落后远程 n 个提交

#### 4. 使用情况段 (📊)
- 显示上下文窗口使用百分比
- 显示当前 token 数量
- 基于转录文件中的最新使用情况

## ⚙️ 配置选项

### 查看默认配置
```bash
~/.claude/ccline/ccline --print-config
```

输出：
```toml
theme = "dark"

[segments]
directory = true
git = true
model = true
time = false
usage = true
```

### 命令行选项

```bash
# 显示帮助
~/.claude/ccline/ccline --help

# 使用特定主题
~/.claude/ccline/ccline --theme light

# 打印配置
~/.claude/ccline/ccline --print-config

# 验证配置（计划中）
~/.claude/ccline/ccline --validate

# TUI 配置模式（计划中）
~/.claude/ccline/ccline --configure
```

## 🎨 自定义和扩展

### 启用时间段
如果你想显示当前时间，可以修改默认配置：

1. 编辑 `src/config/defaults.rs`
2. 将 `time: false` 改为 `time: true`
3. 重新构建：`cargo build --release`
4. 重新安装到系统

### 颜色自定义
当前颜色方案：
- 🤖 模型: 亮青色 (`\x1b[1;36m`)
- 📁 目录: 黄色图标 + 绿色文本
- 🌿 Git: 亮蓝色 (`\x1b[1;34m`)
- ⏰ 时间: 亮青色 (`\x1b[1;36m`)
- 📊 使用情况: 亮紫色 (`\x1b[1;35m`)
- 分隔符: 白色 (`\x1b[37m`)

## 🐛 故障排除

### 常见问题

#### 1. 状态栏不显示
- 检查 Claude Code 配置是否正确
- 确认可执行文件路径和权限
- 验证输入数据格式

#### 2. Git 信息不显示
- 确保当前目录是 Git 仓库
- 检查 Git 命令是否可用：`git --version`

#### 3. 使用情况显示为 0
- 检查转录文件路径是否正确
- 确认转录文件格式符合预期

#### 4. 性能问题
- 检查 Git 仓库大小（大仓库可能影响性能）
- 确认没有网络相关的阻塞操作

### 调试模式

```bash
# 启用调试输出（如果编译时包含）
RUST_LOG=debug cat test_input.json | ~/.claude/ccline/ccline

# 检查输入数据
cat test_input.json | jq .

# 手动测试各个组件
git status --porcelain  # 测试 Git 状态
pwd                     # 测试目录获取
```

## 📈 性能特点

- **启动时间**: < 50ms
- **内存使用**: < 10MB
- **CPU 使用**: 极低，主要是 I/O 操作
- **二进制大小**: ~2MB（优化构建）

## 🔄 更新和维护

### 更新程序
```bash
cd CCometixLine
git pull
cargo build --release
cp target/release/ccometixline ~/.claude/ccline/ccline
```

### 备份配置
```bash
# 如果有自定义配置文件
cp ~/.claude/ccline/config.toml ~/.claude/ccline/config.toml.backup
```

这个工具设计得很轻量且高效，适合日常开发使用！