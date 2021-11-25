#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::cell::Cell;
use anyhow::{anyhow, bail};
use app::error::Result;
use app::{Account, User};
use app::video::{BiliBili, Client, LoginInfo, Studio, Video};
use tauri::{Manager, Window};

#[tauri::command]
fn login(username: &str, password: &str, remember_me: bool) -> Result<String> {
  lg(username, password)?;
  let file = std::fs::File::create("config.yaml")?;
  if remember_me {
    serde_yaml::to_writer(file, &User{
      account: Account{ username: username.into(), password: password.into() }
    })?
  }
  // println!("body = {:?}", client);
  Ok("登录成功".into())
}

#[tauri::command]
async fn login_by_cookie() -> Result<String> {
  let file = std::fs::File::open("cookies.json")?;
  Client::new().login_by_cookies(file).await?;
  // println!("body = {:?}", client);
  Ok("登录成功".into())
}

#[tauri::command]
async fn upload(mut video: Video, window: Window) -> Result<(Video, f64)> {
  let mut client = Client::new();
  let login_info = client.login_by_cookies(std::fs::File::open("cookies.json")?).await?;
  let bili = BiliBili::new(
    (client, login_info)
  );
    // let videos = &self.studio.videos;
    // let mut new_videos = Vec::with_capacity(videos.len());
  // if studio.videos.is_empty() { return Err(app::error::Error::Err("文件不能为空".into())) }
  // for video in &mut studio.videos {
    let mut total_size = 0;
    let mut speed= 0.;
    video = bili.upload_file(&video.filename,  |instant, total, size|{
      window.emit("progress", (&video.filename, total_size as f64/ total as f64 * 100., total_size as f64 / 1000. / instant.elapsed().as_millis() as f64)).unwrap();
      total_size+=size;
      speed = total_size as f64 / 1000. / instant.elapsed().as_millis() as f64;
      println!("{:.2}% => {:.2} MB/s.", total_size as f64/ total as f64 * 100., speed);
    }).await?;
    // new_videos.push(video);
  // }

  println!("上传成功");
  Ok((video, speed))
}

#[tauri::command]
async fn submit(studio: Studio) -> Result<serde_json::Value> {
  let mut client = Client::new();
  let login_info = client.login_by_cookies(std::fs::File::open("cookies.json")?).await?;
  let bili = BiliBili::new(
    (client, login_info)
  );
  let mut bilibili = bili.submit(studio).await?;
  Ok(bilibili)
}

#[tauri::command]
fn load_account() -> Result<User> {
  let file = std::fs::File::open("config.yaml")?;
  let user: User = serde_yaml::from_reader(file)?;
  // println!("body = {:?}", client);
  Ok(user)
}

#[tokio::main]
async fn lg(username: &str, password: &str) -> anyhow::Result<(Client, LoginInfo)> {
  Client::new().login(username, password).await
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![login, login_by_cookie, load_account, upload, submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
