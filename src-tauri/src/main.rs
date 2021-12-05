#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::{anyhow, bail, Context};
use app::error::Result;
use app::{Account, Config, User};
use app::video::{BiliBili, Client, LoginInfo, Studio, Video};
use tauri::{Manager, Window};

#[tauri::command]
fn login(username: &str, password: &str, remember_me: bool) -> Result<String> {
  login_by_password(username, password)?;
  if remember_me {
    match load() {
      Ok(mut config) => {
        let file = std::fs::File::create("config.yaml").with_context(||0)?;
        config.user.account.username = username.into();
        config.user.account.password = password.into();
        serde_yaml::to_writer(file, &config).with_context(||1)?
      }
      Err(_) => {
        let file = std::fs::File::create("config.yaml").with_context(||2)?;
        serde_yaml::to_writer(file, &Config{
          user: User {account: Account{ username: username.into(), password: password.into() }},
          streamers: Default::default()
        }).with_context(||3)?
      }
    }
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
  let remove = Arc::new(AtomicBool::new(false));
  let is_remove = Arc::clone(&remove);
  let id = window.once(&video.filename, move |event| {
    println!("got window event-name with payload {:?}", event.payload());
    is_remove.store(true, Ordering::Relaxed);
  });
    let mut total_size = 0;
    let mut speed= 0.;
    video = bili.upload_file(&video.filename,  |instant, total, size|{
      window.emit("progress", (&video.filename, total_size as f64/ total as f64 * 100., total_size as f64 / 1000. / instant.elapsed().as_millis() as f64)).unwrap();
      total_size+=size;
      speed = total_size as f64 / 1000. / instant.elapsed().as_millis() as f64;
      println!("{:.2}% => {:.2} MB/s.", total_size as f64/ total as f64 * 100., speed);
      println!("{}", remove.load(Ordering::Relaxed));
      remove.load(Ordering::Relaxed)
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
async fn archive_pre() -> Result<serde_json::Value> {
  let mut client = Client::new();
  let login_info = client.login_by_cookies(std::fs::File::open("cookies.json")?).await?;
  let bili = BiliBili::new(
    (client, login_info)
  );
  let mut bilibili = bili.archive_pre().await?;
  Ok(bilibili)
}

#[tauri::command]
fn load_account() -> Result<User> {
  let file = std::fs::File::open("config.yaml")?;
  let user: User = serde_yaml::from_reader(file)?;
  // println!("body = {:?}", client);
  Ok(user)
}

#[tauri::command]
fn load() -> Result<Config> {
  let file = std::fs::File::open("config.yaml")?;
  let config: Config = serde_yaml::from_reader(file)?;
  // println!("body = {:?}", client);
  Ok(config)
}

#[tauri::command]
fn save(config: Config) -> Result<Config> {
  let file = std::fs::File::create("config.yaml")?;
  // let config: Config = serde_yaml::from_reader(file)?;
  serde_yaml::to_writer(file, &config);
  // println!("body = {:?}", client);
  Ok(config)
}

#[tokio::main]
async fn login_by_password(username: &str, password: &str) -> anyhow::Result<LoginInfo> {
  Client::new().login_by_password(username, password).await
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![login, login_by_cookie, load_account, upload, submit, archive_pre, load, save])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
