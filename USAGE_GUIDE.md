# Zed 翻译扩展使用指南

## 🚀 快速开始

### 方法 1: 使用 AI 面板（推荐）

1. **选中要翻译的文本**
2. **打开 AI 面板**：
   - Mac: `Cmd+Shift+A` 或点击右侧 AI 图标
   - Windows/Linux: `Ctrl+Shift+A`
3. **输入翻译命令**：
   ```
   /translate zh Hello World
   /translate-zh How are you?
   /translate-en 你好世界
   ```

### 方法 2: 使用命令面板

1. **打开命令面板**：
   - Mac: `Cmd+Shift+P`
   - Windows/Linux: `Ctrl+Shift+P`
2. **输入** `translate` 查找翻译相关命令
3. **选择并执行**

## ⌨️ 配置自定义快捷键

### 第一步：打开 Zed 配置

1. 打开命令面板（`Cmd/Ctrl+Shift+P`）
2. 输入 "zed: open keymap"
3. 回车打开 `keymap.json` 文件

### 第二步：添加快捷键配置

将以下配置复制到你的 `~/.config/zed/keymap.json` 文件中：

```json
[
  {
    "context": "Editor",
    "bindings": {
      // 方案 A: 快速打开 AI 面板
      "ctrl-shift-t": "workspace::ToggleRightDock",

      // 方案 B: 快速将选中文本添加到 AI 上下文
      "ctrl-alt-t": "agent::AddSelectionToThread"
    }
  }
]
```

### 推荐的工作流程

#### 工作流 A: AI 面板翻译（最便捷）

```
1. 选中文本
2. 按 Ctrl+Shift+T 打开 AI 面板
3. 输入 /translate-zh 或 /translate-en
4. 按 Enter 获取翻译结果
```

#### 工作流 B: 上下文翻译

```
1. 选中文本
2. 按 Ctrl+Alt+T 将文本添加到 AI 上下文
3. 在 AI 面板中输入翻译命令
4. 查看翻译结果
```

## 🎯 支持的命令

### `/translate [语言代码] <文本>`

通用翻译命令，可以翻译到任何支持的语言。

**示例：**
```
/translate zh Hello World
→ 翻译到中文：你好世界

/translate ja Good morning
→ 翻译到日语：おはようございます

/translate fr Thank you
→ 翻译到法语：Merci
```

**支持的语言代码：**
- `en` - English (英语)
- `zh` - Chinese (中文)
- `ja` - Japanese (日语)
- `ko` - Korean (韩语)
- `fr` - French (法语)
- `de` - German (德语)
- `es` - Spanish (西班牙语)
- `ru` - Russian (俄语)
- `it` - Italian (意大利语)
- `pt` - Portuguese (葡萄牙语)

### `/translate-zh <文本>`

快速翻译到中文的便捷命令。

**示例：**
```
/translate-zh Hello, how are you?
→ 翻译结果：你好，你好吗？

/translate-zh The weather is nice today
→ 翻译结果：今天天气很好
```

### `/translate-en <文本>`

快速翻译到英语的便捷命令。

**示例：**
```
/translate-en 今天天气真好
→ Translation: The weather is really nice today

/translate-en 你好，最近怎么样？
→ Translation: Hello, how have you been recently?
```

## 💡 使用技巧

### 技巧 1: 批量翻译

你可以一次翻译多段文本：

```
/translate-zh
Hello World
Good morning
Thank you
```

### 技巧 2: 自动语言检测

`/translate` 命令会自动检测源语言：

```
/translate en 你好
→ 自动检测中文，翻译为 "Hello"

/translate zh Hello
→ 自动检测英语，翻译为 "你好"
```

### 技巧 3: 代码注释翻译

翻译代码注释时，选中注释文本即可：

```javascript
// 选中这段注释
// /translate-en 这是一个示例函数
function example() {
  // ...
}
```

### 技巧 4: 多语言文档

在编写多语言文档时快速翻译：

```markdown
# English Version
Hello World

# 中文版本
/translate-zh Hello World
```

## 🔧 自定义配置

### 修改快捷键

如果默认快捷键冲突，可以修改为其他组合：

```json
{
  "context": "Editor",
  "bindings": {
    // 使用 Cmd+T (Mac) 或 Ctrl+T (Windows/Linux)
    "cmd-t": "workspace::ToggleRightDock",

    // 或者使用 Alt+T
    "alt-t": "workspace::ToggleRightDock",

    // 或者使用 F2
    "f2": "agent::AddSelectionToThread"
  }
}
```

### 查看所有可用命令

在命令面板中输入 `>` 可以查看所有 Zed 命令。

## 🐛 常见问题

### Q: 快捷键不工作？

**A:** 检查以下几点：
1. 确保配置文件格式正确（valid JSON）
2. 重启 Zed 编辑器
3. 检查是否与其他快捷键冲突

### Q: 找不到翻译命令？

**A:** 确保：
1. 扩展已正确安装
2. 在 AI 面板中使用（而非普通编辑器）
3. 命令以 `/` 开头

### Q: 如何查看所有 slash commands？

**A:** 在 AI 面板中输入 `/`，会自动显示所有可用的 slash commands。

### Q: 翻译结果不准确？

**A:** 当前版本使用模拟翻译。真实的翻译功能需要：
1. 配置翻译 API（Google、DeepL 等）
2. 添加 API key
3. 参考 README 中的开发路线图

## 📚 更多资源

- [Zed 官方文档](https://zed.dev/docs)
- [Zed 快捷键参考](https://zed.dev/docs/key-bindings)
- [扩展开发文档](https://zed.dev/docs/extensions)

## 🤝 反馈与支持

如果你有任何问题或建议，欢迎：
- 提交 GitHub Issue
- 参与讨论
- 贡献代码

---

**提示**: 这个扩展正在积极开发中。未来版本将支持真实的翻译 API 集成！
