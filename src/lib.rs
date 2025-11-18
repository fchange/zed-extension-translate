use serde::{Deserialize, Serialize};
use std::fs;
use zed_extension_api::{self as zed, Result, SlashCommand, SlashCommandOutput, SlashCommandOutputSection};

struct TranslateExtension {
    cached_binary_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationRequest {
    text: String,
    source: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleTranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

impl TranslateExtension {
    fn translate_text(&self, text: &str, target_lang: &str) -> Result<String> {
        // 使用 Google Translate API (免费版本，无需 API key)
        // 这里使用简单的 HTTP 请求
        let encoded_text = urlencoding::encode(text);
        let url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
            target_lang, encoded_text
        );

        // 由于 Zed 扩展的限制，我们在这里使用一个简化的实现
        // 实际使用时需要通过 HTTP 客户端调用

        // 返回模拟的翻译结果（在实际实现中应该调用真实的 API）
        Ok(format!("[Translation to {}]: {}", target_lang, text))
    }

    fn get_language_code(lang: &str) -> &str {
        match lang.to_lowercase().as_str() {
            "chinese" | "zh" | "中文" => "zh-CN",
            "english" | "en" | "英文" => "en",
            "japanese" | "ja" | "日文" => "ja",
            "korean" | "ko" | "韩文" => "ko",
            "french" | "fr" | "法文" => "fr",
            "german" | "de" | "德文" => "de",
            "spanish" | "es" | "西班牙文" => "es",
            "russian" | "ru" | "俄文" => "ru",
            "italian" | "it" | "意大利文" => "it",
            "portuguese" | "pt" | "葡萄牙文" => "pt",
            _ => lang,
        }
    }
}

impl zed::Extension for TranslateExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn slash_command_completions(&mut self, _command: SlashCommand) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn run_slash_command(
        &mut self,
        command: SlashCommand,
        _output: &mut dyn zed::io::Write,
    ) -> Result<SlashCommandOutput> {
        match command.name.as_str() {
            "translate" => {
                let args: Vec<&str> = command.argument.split_whitespace().collect();

                let (target_lang, text) = if args.is_empty() {
                    ("en", command.argument.trim())
                } else {
                    let lang = Self::get_language_code(args[0]);
                    let remaining_text = args[1..].join(" ");
                    if remaining_text.is_empty() {
                        (lang, command.argument.trim())
                    } else {
                        (lang, remaining_text.as_str())
                    }
                };

                if text.is_empty() {
                    return Ok(SlashCommandOutput {
                        sections: vec![SlashCommandOutputSection {
                            range: (0..0),
                            label: "Error".to_string(),
                        }],
                        text: "Usage: /translate [language] <text>\nExample: /translate zh Hello World\nSupported languages: en, zh, ja, ko, fr, de, es, ru, it, pt".to_string(),
                    });
                }

                let result = self.translate_text(text, target_lang)
                    .unwrap_or_else(|e| format!("Translation error: {:?}", e));

                let output_text = format!(
                    "Original: {}\nTarget Language: {}\nTranslation: {}",
                    text, target_lang, result
                );

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()),
                        label: format!("Translation ({})", target_lang),
                    }],
                    text: output_text,
                })
            }
            "translate-zh" => {
                let text = command.argument.trim();
                if text.is_empty() {
                    return Ok(SlashCommandOutput {
                        sections: vec![],
                        text: "Usage: /translate-zh <text>".to_string(),
                    });
                }

                let result = self.translate_text(text, "zh-CN")
                    .unwrap_or_else(|e| format!("Translation error: {:?}", e));

                let output_text = format!("翻译结果: {}", result);

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()),
                        label: "Chinese Translation".to_string(),
                    }],
                    text: output_text,
                })
            }
            "translate-en" => {
                let text = command.argument.trim();
                if text.is_empty() {
                    return Ok(SlashCommandOutput {
                        sections: vec![],
                        text: "Usage: /translate-en <text>".to_string(),
                    });
                }

                let result = self.translate_text(text, "en")
                    .unwrap_or_else(|e| format!("Translation error: {:?}", e));

                let output_text = format!("Translation: {}", result);

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()),
                        label: "English Translation".to_string(),
                    }],
                    text: output_text,
                })
            }
            command => Err(format!("Unknown slash command: {}", command)),
        }
    }
}

// 简单的 URL 编码模块
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

zed::register_extension!(TranslateExtension);
