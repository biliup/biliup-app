use std::path::PathBuf;
use biliup::{Config, User};
use anyhow::Context;
use biliup::client::{Client, LoginInfo};


pub mod error;

pub fn config_file() -> error::Result<PathBuf> {
    Ok(config_path()?.join("config.yaml"))
}

pub fn cookie_file() -> error::Result<PathBuf> {
    Ok(config_path()?.join("cookies.json"))
}

pub fn config_path() -> error::Result<PathBuf> {
    let mut config_dir = tauri::api::path::config_dir().ok_or(error::Error::Err("config_dir".to_string()))?;
    config_dir.push("biliup");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    println!("{config_dir:?}");
    Ok(config_dir)
}


#[tokio::main]
pub async fn login_by_password(username: &str, password: &str) -> anyhow::Result<()> {
    let info = Client::new().login_by_password(username, password).await?;
    let file = std::fs::File::create(cookie_file()?)?;
    serde_json::to_writer_pretty(&file, &info)?;
    println!("密码登录成功，数据保存在{:?}", file);
    Ok(())
}

pub async fn login_by_cookies() -> anyhow::Result<(LoginInfo, Client)> {
    let client = Client::new();
    let login_info = client
        .login_by_cookies(std::fs::File::open(cookie_file()?)?)
        .await?;
    Ok((login_info, client))
}
