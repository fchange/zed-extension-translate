# Zed Translation Extension

A powerful translation extension for Zed editor that brings real-time text translation directly into your development environment.

## Features

- 🌍 **Multiple Language Support**: Translate text between various languages including English, Chinese, Japanese, Korean, French, German, Spanish, Russian, Italian, and Portuguese
- ⚡ **Quick Access**: Use simple slash commands to translate text instantly
- 🎯 **Smart Language Detection**: Automatically detects source language
- 💡 **Developer Friendly**: Designed specifically for developers who work with multilingual codebases and documentation

## Installation

### From Source (Development)

1. Clone this repository:
```bash
git clone https://github.com/your-username/zed-extension-translate.git
cd zed-extension-translate
```

2. In Zed, open the Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`)
3. Click on "Install Dev Extension"
4. Select the directory containing this extension

### From Zed Extensions Registry

Once published, you can install it directly from Zed:
1. Open Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`)
2. Search for "Translate"
3. Click Install

## Usage

This extension provides three slash commands for quick translation:

### `/translate [language] <text>`

Translate text to any supported language.

**Examples:**
```
/translate zh Hello World
→ Translates "Hello World" to Chinese

/translate ja Good morning
→ Translates "Good morning" to Japanese

/translate en 你好世界
→ Translates "你好世界" to English
```

### `/translate-zh <text>`

Quick command to translate text to Chinese.

**Example:**
```
/translate-zh Hello, how are you?
→ 你好，你好吗？
```

### `/translate-en <text>`

Quick command to translate text to English.

**Example:**
```
/translate-en 今天天气真好
→ The weather is really nice today
```

## Supported Languages

- **English** (en)
- **Chinese** (zh, zh-CN)
- **Japanese** (ja)
- **Korean** (ko)
- **French** (fr)
- **German** (de)
- **Spanish** (es)
- **Russian** (ru)
- **Italian** (it)
- **Portuguese** (pt)

## Use Cases

- 📝 **Documentation Translation**: Quickly understand code comments in different languages
- 🔍 **Code Review**: Translate commit messages and pull request descriptions
- 📚 **Learning**: Understand foreign language tutorials and documentation
- 🌏 **International Teams**: Communicate effectively with team members worldwide
- 💬 **Quick Reference**: Translate error messages and stack traces

## Development

### Prerequisites

- [Rust](https://rustup.rs/) installed via rustup
- Zed editor

### Building

```bash
cargo build --release
```

### Testing Locally

1. Start Zed in foreground mode for detailed logs:
```bash
zed --foreground
```

2. Install the extension as a dev extension (see Installation section)

### Project Structure

```
zed-extension-translate/
├── extension.toml          # Extension metadata
├── Cargo.toml             # Rust dependencies
├── src/
│   └── lib.rs            # Main extension code
├── slash_commands/        # Slash command definitions
│   ├── translate.toml
│   ├── translate-zh.toml
│   └── translate-en.toml
└── README.md
```

## Roadmap

- [ ] Add more translation engines (DeepL, OpenAI, etc.)
- [ ] Implement caching for faster repeated translations
- [ ] Add text-to-speech functionality
- [ ] Support for custom API keys
- [ ] Translation history and wordbook
- [ ] Batch translation support
- [ ] In-editor documentation translation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Inspiration

This extension is inspired by the excellent [Translation Plugin](https://github.com/YiiGuxing/TranslationPlugin) for JetBrains IDEs.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Zed team for creating an extensible editor
- Inspired by Translation Plugin for JetBrains IDEs
- Built with ❤️ for developers working across languages
