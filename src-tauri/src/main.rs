// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::multipart;
use rfd::AsyncFileDialog;
use serde::Serialize;
use std::env;
use std::io::Write;
use std::{fs::File, io::Read};
use tokio::io::AsyncReadExt;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_image, matting_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct ImageParam {
    image_base64: String,
    image_path: String,
}

/**
 * 打开图片
 */
#[tauri::command(async)]
async fn open_image() -> Result<ImageParam, String> {
    //打开图片
    let file = AsyncFileDialog::new()
        .add_filter("image", &["png", "jpeg", "jpg"])
        .set_directory("/")
        .pick_file()
        .await;

    let image_path = file.unwrap().path().to_str().unwrap().to_string();
    //读取图片转换为base64
    return image_to_base64(image_path);
}

/** 抠图 */
#[tauri::command(async)]
async fn matting_image(file_path: String) -> Result<ImageParam, String> {
    println!("开始抠图：{:?}", file_path);
    let mut file = tokio::fs::File::open(file_path).await.unwrap();

    let mut contents = Vec::new();

    // 读取文件内容
    let _ = file.read_to_end(&mut contents).await;

    // 构建 multipart form data
    let form = multipart::Form::new()
        .part(
            "image_file",
            multipart::Part::bytes(contents).file_name("file.jpg"),
        )
        .text("size", "auto");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.remove.bg/v1.0/removebg")
        .header("X-Api-Key", "你创建的密钥")
        .multipart(form)
        .send()
        .await;

    match response {
        Ok(res) => {
            // 从响应中获取图片数据
            let image_data = res.bytes().await.unwrap();
            //获取项目目录
            let save_path = "F:/images/image.jpg".to_string();
            println!("save:{:?}",save_path);
            // 将图片数据保存到文件
            let mut file = std::fs::File::create(&save_path).unwrap();
            let _ = file.write_all(&image_data);
            println!("保存成功!");
            return image_to_base64(save_path);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    Err("err".to_string())
}



/** 图片转换对象，包含base64 和 图片根路径 */
fn image_to_base64(save_path: String) -> Result<ImageParam, String> {
    // 打开文件
    let mut file = match File::open(&save_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };
    // 存放读取文件
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            // 读取的内容转换成base64
            let base64_image = base64::encode(&buffer);
            let mime_type = "image/jpeg";
            let base64_with_prefix = format!("data:{};base64,{}", mime_type, base64_image);
            return Ok(ImageParam {
                image_base64: base64_with_prefix,
                image_path: save_path,
            });
        }
        Err(err) => {
            println!("err = {:?}",err);
            return Err(format!("{:?}", err));
        }
    }
}