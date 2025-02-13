mod dao;
use dao::CHAPTERS;

use rand::Rng;
use serde_json::json;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_question() -> serde_json::Value {
    // 使用固定种子生成随机数（调试用）
    let mut rng = rand::thread_rng();

    let row = rng.gen_range(0..9);     // 随机行号 (0-8)
    let col = rng.gen_range(1..=9);    // 随机列号 (1-9)
    let chapter = row * 9 + col;      // 计算章节号

    // 获取对应的章节内容
    let chapter_text = CHAPTERS[chapter.min(CHAPTERS.len() - 1) as usize - 1];

    // 分割章节内容
    let sections = auto_split(chapter_text);

    // 构造 JSON 响应
    json!({
        "id": chapter,
        "title": format!("{} 行 {} 列: 第 {} 章", row + 1, col, chapter),
        "firstSentence": sections.get(0).unwrap_or(&"".to_string()),
    })
}

#[tauri::command]
fn check_answer(chapter: usize, answer: &str) -> serde_json::Value {
    let chapter_index = chapter - 1;
    let chapter_text = CHAPTERS[chapter_index];
    
    let user_answer = auto_split(answer);
    let correct_anwser_full = auto_split(chapter_text);
    let correct_anwser = &correct_anwser_full[1..];
    
    let is_correct = user_answer.as_slice() == correct_anwser;
    let content = correct_anwser.join("   ");
    
    json!({
        "correct": is_correct,
        "content": content,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_question, check_answer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn auto_split(text: &str) -> Vec<String> {
    // 去掉所有非字母数字和空白字符的符号，并按空格分割
    let mut sections = Vec::new();
    let tokens: Vec<char> = text.chars().collect();
    let mut current_token = String::new();

    for c in tokens {
        if c.is_alphanumeric() || c.is_whitespace() {
            current_token.push(c);
        } else {
            current_token.push(' ');
        }
    }

    for part in current_token.split_whitespace() {
        sections.push(part.to_string());
    }

    sections
}
