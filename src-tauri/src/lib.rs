use std::path::PathBuf;
use biliup::{Config, User};
use anyhow::Context;
use biliup::client::{Client, LoginInfo};
use std::fmt::Write;
use std::pin::Pin;
use std::task::Poll;
use futures::{FutureExt, Stream};
use std::future::Future;
use std::rc::Rc;
use reqwest::Body;
use bytes::{Buf, Bytes};
use tauri::{App, Window};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;

pub mod error;

#[derive(Default)]
pub struct Credential {
    pub credential: RwLock<Option<Arc<(LoginInfo, Client)>>>
}

impl Credential {
    pub async fn get_credential(&self) -> error::Result<Arc<(LoginInfo, Client)>> {
        {
            let read_guard = self.credential.read().unwrap();
            if !read_guard.is_none() {
                return Ok(read_guard.as_ref().unwrap().clone());
            }
        }
        let client = Client::new();
        let login_info = client
            .login_by_cookies(std::fs::File::open(cookie_file()?)?)
            .await?;
        let arc = Arc::new((login_info, client));
        *self.credential.write().unwrap() = Some(arc.clone());
        Ok(arc)
    }
}

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
    println!("config_path: {config_dir:?}");
    Ok(config_dir)
}


pub async fn login_by_password(username: &str, password: &str) -> anyhow::Result<()> {
    let info = Client::new().login_by_password(username, password).await?;
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
            pb.send(n as u64);
            Ok(Some(content_bytes.copy_to_bytes(n)))
        } else {
            pb.send(pc as u64);
            Ok(Some(content_bytes.copy_to_bytes(pc)))
        }
    }
}

impl Stream for Progressbar{
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
    fn test_hex() {
    }
}
