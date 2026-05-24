# Sunny Leo

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

Zed 编辑器上的 **Leo** 语言扩展，提供语法高亮、代码折叠、缩进等语言支持。

Leo 是一种为零知识证明应用设计的编程语言，详见 [Aleo 官方文档](https://developer.aleo.org/leo)。

## 功能特性

- 🎨 **语法高亮** – 基于 tree-sitter 的精准着色
- 📁 **代码折叠** – 支持按语法结构折叠代码块
- 📏 **智能缩进** – 根据代码上下文自动缩进

### 待开发

- 🔍 **代码大纲** – locals 查询支持符号导航
- ✏️ **文本对象** – 支持结构化文本选择
- 🧠 **语言服务器（LSP）** – 补全、诊断、跳转等智能特性
- 🎨 **代码格式化** – 自动排版

## 安装

### 1. 克隆仓库

```powershell
git clone https://github.com/HirongVSuen/sunny-leo.git
cd sunny-leo
```

### 2. 安装到 Zed

在 Zed 中按 `Ctrl+Shift+P` 打开命令面板，输入 `zed: install dev` 并回车。

### 3. 重启 Zed

重启后打开任意 `.leo` 文件即可生效。

## 项目结构

```
sunny-leo/
├── src/
│   └── lib.rs              # 扩展入口
├── languages/leo/
│   ├── config.toml          # 语言配置
│   ├── highlights.scm       # 语法高亮规则
│   ├── folds.scm            # 代码折叠规则
│   ├── indents.scm          # 缩进规则
│   ├── locals.scm           # 局部符号查询
│   └── textobjects.scm      # 文本对象定义
├── grammars/leo/            # tree-sitter 语法（源码及 query）
│   ├── src/                 # 语法解析器 C 源码
│   └── queries/             # tree-sitter 查询文件
├── Cargo.toml               # Rust 项目配置
├── extension.toml           # Zed 扩展清单
└── extension.wasm           # 编译产物
```

## 许可证

[Apache License 2.0](LICENSE)
