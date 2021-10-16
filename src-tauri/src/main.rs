#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use app::error::Result;

#[tauri::command]
async fn test() -> Result<String> {
  let body = reqwest::get("https://www.rust-lang.org")
      .await?
      .text()
      .await?;

  println!("body = {:?}", body);
  Ok(body)
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
