use bip39::{Language, Mnemonic};
use rand::thread_rng;

/// 使用指定的语言生成一个 12 个单词的助记词
/// 注意：Language 枚举在生成时是可选的，
pub fn generate(language: Language) -> Mnemonic {
    Mnemonic::generate_in(Language::English, 12).unwrap()
}
