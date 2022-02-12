#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::cell::Cell;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use anyhow::{anyhow, bail, Context};
use biliup::{Account, Config, line, User};
use biliup::client::{Client, LoginInfo};
use biliup::video::{BiliBili, Studio, Video};
use tauri::{Manager, Window};

// use app::video::{BiliBili, Client, LoginInfo, Studio, Video};
use app::{config_file, cookie_file, encode_hex, login_by_cookies};
use app::error;
use app::error::Result;

#[tauri::command]
fn login(username: &str, password: &str, remember_me: bool) -> Result<String> {
    app::login_by_password(username, password)?;
    if remember_me {
        match load() {
            Ok(mut config) => {
                if let Some(ref mut user) = config.user {
                    user.account.username = username.into();
                    user.account.password = password.into();
                }
                save(config)?;
                // let file = std::fs::File::create(config_file()?).with_context(|| "open config.yaml")?;
                // serde_yaml::to_writer(file, &config).with_context(|| "write config.yaml")?
            }
            Err(_) => {
                // let file = std::fs::File::create("config.yaml").with_context(|| "create config.yaml")?;
                save(Config {
                    user: Some(User {
                        account: Account {
                            username: username.into(),
                            password: password.into(),
                        },
                    }),
                    line: None,
                    limit: 3,
                    streamers: Default::default(),
                })?;
            }
        }
    }
    // println!("body = {:?}", client);
    Ok("登录成功".into())
}

#[tauri::command]
async fn login_by_cookie() -> Result<String> {
    login_by_cookies().await?;
    // println!("body = {:?}", client);
    Ok("登录成功".into())
}

#[tauri::command]
async fn login_by_sms(code: u32, res: serde_json::Value) -> Result<String> {
    let info = Client::new().login_by_sms(code, res).await?;
    let file = std::fs::File::create(cookie_file()?)?;
    serde_json::to_writer_pretty(&file, &info)?;
    println!("短信登录成功，数据保存在{:?}", file);
    Ok("登录成功".into())
}

#[tauri::command]
async fn send_sms(country_code: u32, phone: u64) -> Result<serde_json::Value> {
    let ret = Client::new().send_sms(phone, country_code).await?;
    // println!("body = {:?}", client);
    Ok(ret)
}

#[tauri::command]
async fn login_by_qrcode(res: serde_json::Value) -> Result<String> {
    let info = Client::new().login_by_qrcode(res).await?;
    let file = std::fs::File::create(cookie_file()?)?;
    serde_json::to_writer_pretty(&file, &info)?;
    println!("链接登录成功，数据保存在{:?}", file);
    Ok("登录成功".into())
}

#[tauri::command]
async fn get_qrcode() -> Result<serde_json::Value> {
    let qrcode = Client::new().get_qrcode().await?;
    // println!("body = {:?}", client);
    Ok(qrcode)
}


#[tauri::command]
async fn upload(mut video: Video, window: Window) -> Result<(Video, f64)> {
    let (_, client) = login_by_cookies().await?;

    let config = load()?;
    let probe = if let Some(line) = config.line {
        match line.as_str() {
            "kodo" => line::kodo(),
            "bda2" => line::bda2(),
            "ws" => line::ws(),
            "qn" => line::qn(),
            _ => unreachable!()
        }
    } else {
        line::Probe::probe().await?
    };
    let limit = config.limit;
    let remove = Arc::new(AtomicBool::new(true));
    let is_remove = Arc::clone(&remove);
    let id = window.once(encode_hex(video.filename.encode_utf16().collect::<Vec<u16>>().as_slice()), move |event| {
        println!("got window event-name with payload {:?}", event.payload());
        is_remove.store(false, Ordering::Relaxed);
    });
    let mut uploaded = 0;
    let mut speed = 0.;

    let filename = video.filename;
    let filepath = PathBuf::from(&filename);
    let parcel = probe.to_uploader(&filepath).await?;
    let total_size = parcel.total_size;
    let instant = Instant::now();
    video = parcel.upload(&client, limit, |len| {
        window
            .emit(
                "progress",
                (
                    &filename,
                    uploaded,
                    total_size,
                    uploaded as f64 / 1000. / instant.elapsed().as_millis() as f64,
                ),
            )
            .unwrap();
        uploaded += len;
        speed = uploaded as f64 / 1000. / instant.elapsed().as_millis() as f64;
        println!(
            "{:.2}% => {:.2} MB/s.",
            uploaded as f64 / total_size as f64 * 100.,
            speed
        );
        println!("{}", remove.load(Ordering::Relaxed));
        remove.load(Ordering::Relaxed)
    }).await?;
    println!("上传成功");
    Ok((video, speed))
}

#[tauri::command]
async fn submit(mut studio: Studio) -> Result<serde_json::Value> {
    let (login_info, _) = login_by_cookies().await?;
    let ret = studio.submit(&login_info).await?;
    // let bili = BiliBili::new((client, login_info));
    // let mut bilibili = bili.submit(studio).await?;
    Ok(ret)
}

#[tauri::command]
async fn archive_pre() -> Result<serde_json::Value> {
    let (login_info, client) = login_by_cookies().await?;
    let bili = BiliBili::new(&login_info, &client);
    Ok(bili.archive_pre().await?)
}

#[tauri::command]
async fn get_myinfo() -> Result<serde_json::Value> {
    let (_, client) = login_by_cookies().await?;
    Ok(client.client.get("https://api.bilibili.com/x/space/myinfo").send().await?.json().await?)
}

#[tauri::command]
fn load_account() -> Result<User> {
    // let file = std::fs::File::open("config.yaml")?;
    // let user: User = serde_yaml::from_reader(file)?;
    // println!("body = {:?}", client);
    Ok(load()?.user.ok_or(error::Error::Err("账号信息不存在".into()))?)
}

#[tauri::command]
fn save(config: Config) -> Result<Config> {
    let file = std::fs::File::create(config_file()?)?;
    // let config: Config = serde_yaml::from_reader(file)?;
    serde_yaml::to_writer(file, &config);
    // println!("body = {:?}", client);
    Ok(config)
}

#[tauri::command]
fn load() -> Result<Config> {
    let file = std::fs::File::open(config_file()?).with_context(|| "biliup/config.yaml")?;
    let config: Config = serde_yaml::from_reader(file)?;
    // println!("body = {:?}", client);
    Ok(config)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login,
            login_by_cookie,
            load_account,
            upload,
            submit,
            archive_pre,
            load,
            save,
            login_by_sms,
            send_sms,
            login_by_qrcode,
            get_qrcode,
            get_myinfo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
