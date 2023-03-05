use std::path::PathBuf;

use biliup::uploader::credential::{login_by_cookies, Credential as BiliCredential};
use futures::Stream;
use std::fmt::Write;
use std::pin::Pin;
use std::task::Poll;

use bytes::{Buf, Bytes};
use reqwest::Body;

use tokio::sync::mpsc::UnboundedSender;

use std::sync::{Arc, RwLock};
use biliup::uploader::bilibili::BiliBili;

pub mod error;

#[derive(Default)]
pub struct Credential {
    pub credential: RwLock<Option<Arc<BiliBili>>>,
}

impl Credential {
    pub async fn get_credential(&self) -> error::Result<Arc<BiliBili>> {
        {
            let read_guard = self.credential.read().unwrap();
            if !read_guard.is_none() {
                return Ok(read_guard.as_ref().unwrap().clone());
            }
        }
        let login_info = login_by_cookies(cookie_file()?).await?;
        let myinfo: serde_json::Value = login_info
            .client
            .get("https://api.bilibili.com/x/space/myinfo")
            .send()
            .await?
            .json()
            .await?;
        let user = config_path()?.join(format!("users/{}.json", myinfo["data"]["mid"]));
        user_path(user).await?;
        let arc = Arc::new(login_info);
        *self.credential.write().unwrap() = Some(arc.clone());
        Ok(arc)
    }

    pub fn clear(&self) {
        *self.credential.write().unwrap() = None;
    }
}

pub fn config_file() -> error::Result<PathBuf> {
    Ok(config_path()?.join("config.yaml"))
}

pub fn cookie_file() -> error::Result<PathBuf> {
    Ok(config_path()?.join("cookies.json"))
}

pub fn config_path() -> error::Result<PathBuf> {
    let mut config_dir = tauri::api::path::config_dir()
        .ok_or_else(|| error::Error::Err("config_dir".to_string()))?;
    config_dir.push("biliup");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    println!("config_path: {config_dir:?}");
    Ok(config_dir)
}

pub async fn user_path(path: PathBuf) -> error::Result<PathBuf> {
    let mut users = config_path()?;
    users.push("users");
    if !users.exists() {
        std::fs::create_dir(&users)?;
    }
    std::fs::copy(cookie_file()?, &path)?;
    println!("user_path: {path:?}");
    Ok(users)
}

pub async fn login_by_password(username: &str, password: &str) -> anyhow::Result<()> {
    let info = BiliCredential::default().login_by_password(username, password).await?;
    let file = std::fs::File::create(cookie_file()?)?;
    serde_json::to_writer_pretty(&file, &info)?;
    println!("密码登录成功，数据保存在{:?}", file);
    Ok(())
}

pub fn encode_hex(bytes: &[u16]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:x}", b).unwrap();
    }
    s
}

#[derive(Clone)]
pub struct Progressbar {
    bytes: Bytes,
    tx: UnboundedSender<u64>,
    // tx: Sender<u64>,
}

impl Progressbar {
    pub fn new(bytes: Bytes, tx: UnboundedSender<u64>) -> Self {
        Self { bytes, tx }
    }

    pub fn progress(&mut self) -> crate::error::Result<Option<Bytes>> {
        let pb = &self.tx;

        let content_bytes = &mut self.bytes;

        let n = content_bytes.remaining();

        let pc = 1048576;
        if n == 0 {
            Ok(None)
        } else if n < pc {
            pb.send(n as u64)?;
            Ok(Some(content_bytes.copy_to_bytes(n)))
        } else {
            pb.send(pc as u64)?;
            Ok(Some(content_bytes.copy_to_bytes(pc)))
        }
    }
}

impl Stream for Progressbar {
    type Item = crate::error::Result<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.progress()? {
            None => Poll::Ready(None),
            Some(s) => Poll::Ready(Some(Ok(s))),
        }
    }
}

impl From<Progressbar> for Body {
    fn from(async_stream: Progressbar) -> Self {
        Body::wrap_stream(async_stream)
    }
}

mod test {
    #[test]
    fn test_hex() {}
}
