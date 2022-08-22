#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod downloader;
mod douyin;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      douyin::douyin_single_search,
      douyin::douyin_single_download,
      douyin::douyin_muplit_search,
      douyin::douyin_muplit_download,
      douyin::douyin_get_all_video_info,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
