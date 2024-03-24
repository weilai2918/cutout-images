// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Read};

use rfd::AsyncFileDialog;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,open_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command()]
async fn open_image() -> Result<String,String>{
    //打开图片
    let file = AsyncFileDialog::new()
        .add_filter("image", &["png", "jpeg"])
        .set_directory("/")
        .pick_file()
        .await;

    let image_path = file.unwrap().path().to_str().unwrap().to_string();
    //读取图片转换为base64
    let mut file = match File::open(image_path) {
        Ok(file) => file,
        Err(err) =>{
            return Err("读取文件错误".to_string());
        }
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            let base64_image = base64::encode(&buffer);
            let mime_type = "image/jpeg"; 
            let base64_with_prefix = format!("data:{};base64,{}", mime_type, base64_image);
            return Ok(base64_with_prefix);
        }
        Err(err) =>{

        }
    }

    Ok("错误".to_string())
    
}