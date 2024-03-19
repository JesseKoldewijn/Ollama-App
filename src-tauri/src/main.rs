// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tauri::command(rename_all = "snake_case")]
async fn call_ollama2(message: String) {
    let ollama = Ollama::default();

    let model = "llama2:latest".to_string();
    let prompt = message.to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![call_ollama2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
