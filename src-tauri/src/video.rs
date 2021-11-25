use std::cell::RefCell;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use reqwest::{Error, header, Response, Url};
use anyhow::{Result, bail, anyhow};
use async_std::fs::File;
use bytes::{Bytes, BytesMut, BufMut};
use futures::{AsyncReadExt, AsyncWriteExt, Stream, StreamExt, TryStreamExt};
use rand::rngs::OsRng;
use reqwest_cookie_store::CookieStoreMutex;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use rsa::pkcs8::FromPublicKey;
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;
use async_stream::try_stream;
use md5::{Digest, Md5};
use cookie::Cookie;
use base64::{encode};
use url::form_urlencoded;
// use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct Studio {
    #[builder(default=1)]
    copyright: i8,
    source: String,
    #[builder(default=171)]
    tid: i16,
    cover: String,
    #[builder(!default, setter(into))]
    title: String,
    desc_format_id: i8,
    desc: String,
    dynamic: String,
    #[builder(default, setter(skip))]
    subtitle: Subtitle,
    #[builder(default="biliup".into())]
    tag: String,
    #[builder(!default)]
    pub videos: Vec<Video>,
    dtime: Option<i32>,
    open_subtitle: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Subtitle {
    open: i8,
    lan: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    title: Option<String>,
    pub filename: String,
    desc: String
}

impl Video {
    pub fn new(filename: &str) -> Video {
        Video{
            title: None,
            filename: filename.into(),
            desc: "".into()
        }
    }
}

pub struct BiliBili {
    client: reqwest::Client,
    login_info: LoginInfo,
}

impl BiliBili {
    pub fn new((login, login_info): (Client, LoginInfo)) -> BiliBili {
        BiliBili{
            client: login.client,
            login_info
        }
    }

    pub async fn upload_file(&self, filepath: impl std::convert::AsRef<async_std::path::Path>, callback: impl FnMut(Instant, u64, usize)) -> Result<Video> {
        let file = File::open(&filepath).await?;
        let line = Probe::probe().await?;
        let file_name =  filepath.as_ref().file_name().ok_or("No filename").unwrap().to_str();
        let params = json!({
                "r": line.os,
                "profile": "ugcupos/bup",
                "ssl": 0,
                "version": "2.8.12",
                "build": 2081200,
                "name": file_name,
                "size": file.metadata().await?.len(),
            });
        println!("{}", params);
        let res: serde_json::Value = self.client.get(format!("https://member.bilibili.com/preupload?{}", line.query))
            .query(&params).send().await?.json().await?;
        Upos::upload(file, filepath.as_ref(), res, callback).await
    }

    pub async fn submit(&self, studio: Studio) -> Result<serde_json::Value> {
        // studio.videos =
        let ret: serde_json::Value = self.client.post(format!("http://member.bilibili.com/x/vu/client/add?access_key={}", &self.login_info.token_info.access_token))
            .json(&studio).send().await?.json().await?;
        println!("{}", ret);
        if ret["code"] == 0 {
            println!("{}", "投稿成功");
            Ok(ret)
        } else {
            bail!("{}", ret)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Upos<'a> {
    upload_id: &'a str,
    chunks: usize,
    total:u64,
    chunk: usize,
    size: usize,
    part_number: usize,
    start: usize,
    end: usize,
}

impl Upos<'_> {
    async fn upload(file: File, path: &async_std::path::Path, ret: serde_json::Value, mut callback: impl FnMut(Instant, u64, usize)) -> Result<Video> {
        let chunk_size = ret["chunk_size"].as_u64().unwrap() as usize;
        let auth = ret["auth"].as_str().unwrap();
        let endpoint = ret["endpoint"].as_str().unwrap();
        let biz_id = &ret["biz_id"];
        let upos_uri = ret["upos_uri"].as_str().unwrap();
        let url = format!("https:{}/{}", endpoint, upos_uri.replace("upos://", "")); // 视频上传路径
        println!("{}", url);
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Upos-Auth", header::HeaderValue::from_str(auth)?);
        let client = &reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/63.0.3239.108")
            .default_headers(headers)
            .timeout(Duration::new(60, 0))
            .build()
            .unwrap();

        let upload_id: serde_json::Value = client.post(format!("{}?uploads&output=json", url))
            .send().await?.json().await?;
        let upload_id = &upload_id["upload_id"];

        let total_size = file.metadata().await?.len();
        // let parts = Vec::new();
        // let parts_cell = &RefCell::new(parts);
        let chunks_num = ( total_size as f64 / chunk_size as f64).ceil() as usize;  // 获取分块数量
        let url = &url;
        // let file = tokio::io::BufReader::with_capacity(chunk_size, file);

        let instant = Instant::now();
        let mut parts = Vec::with_capacity(chunks_num);
        let mut stream = read_chunk(file, chunk_size)
            // let mut chunks = read_chunk(file, chunk_size)
            .enumerate()
            .map(|( i, chunk)| {
                let chunk = chunk.unwrap();
                let len = chunk.len();
                println!("{}", len);
                let params = Upos {
                    upload_id: upload_id.as_str().unwrap(),
                    chunks: chunks_num,
                    total: total_size,
                    chunk: i,
                    size: len,
                    part_number: i + 1,
                    start: i * chunk_size,
                    end: i * chunk_size + len
                };
                async move {
                    client.put(url).query(&params).body(chunk).send().await?;
                    Ok::<_,reqwest::Error>((json!({"partNumber": params.chunk + 1, "eTag": "etag"}), len))
                }
            }).buffer_unordered(3);
            // .for_each_concurrent()
            // .try_collect().await?;
        // let mut parts = Vec::with_capacity(chunks_num);
        tokio::pin!(stream);
        while let Some((part, size)) = stream.try_next().await? {
            parts.push(part);
            (callback)(instant, total_size, size);
        }
        //     let chunk = chunk.unwrap();
        //     let mut params =  json!({
        //     "uploadId": upload_id,
        //     "chunks": chunks_num,
        //     "total": total_size
        //     });
        //     println!("{}", chunk.len());
        //     params["chunk"] = Value::from(i);
        //     params["size"] = Value::from(chunk.len());
        //     params["partNumber"] = Value::from(i + 1) ;
        //     params["start"] = Value::from(i * chunk_size) ;
        //     params["end"] = Value::from(params["start"].as_u64().unwrap() + params["size"].as_u64().unwrap());
        //     // let body = Body::wrap_stream(chunk);
        //     self.client.put(url).query(&params).body(chunk).header("X-Upos-Auth", auth)
        //         .send().await?;
        //     parts.push(json!({"partNumber": params["chunk"].as_u64().unwrap() + 1, "eTag": "etag"}));
        // }

        // FramedRead::with_capacity(file, BytesCodec::new(), chunk_size)
        //     .enumerate()
        //     .for_each_concurrent(3, |( i, chunk)| async move {
        //         let chunk = chunk.unwrap();
        //     let mut params =  json!({
        //         "uploadId": upload_id,
        //         "chunks": chunks_num,
        //         "total": total_size
        //     });
        //         println!("{}", chunk.len());
        //     params["chunk"] = Value::from(i);
        //     params["size"] = Value::from(chunk.len());
        //     params["partNumber"] = Value::from(i + 1) ;
        //     params["start"] = Value::from(i * chunk_size) ;
        //     params["end"] = Value::from(params["start"].as_u64().unwrap() + params["size"].as_u64().unwrap());
        //         // let body = Body::wrap_stream(chunk);
        //     self.client.put(url).query(&params).body(chunk).header("X-Upos-Auth", auth)
        //         .send().await.unwrap();
        //     parts_cell.borrow_mut().push(json!({"partNumber": params["chunk"].as_u64().unwrap() + 1, "eTag": "etag"}))
        // }).await;
        println!("{:.2} MB/s.", total_size as f64 / 1000. / instant.elapsed().as_millis() as f64);
        // println!("{:?}", parts_cell.borrow());
        let value = json!({
            "name": path.file_name().and_then(OsStr::to_str),
            "uploadId": upload_id,
            "biz_id": biz_id,
            "output": "json",
            "profile": "ugcupos/bup"
        });
        // let res: serde_json::Value = self.client.post(url).query(&value).json(&json!({"parts": *parts_cell.borrow()}))
        let res: serde_json::Value = client.post(url).query(&value).json(&json!({"parts": parts}))
            .send().await?.json().await?;
        if res["OK"] != 1 { bail!("{}", res)}
        Ok(Video{
            title: path.file_stem().and_then(OsStr::to_str).map(|s| s.to_string()),
            filename: Path::new(upos_uri).file_stem().unwrap().to_str().unwrap().into(),
            desc: "".into()
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseData {
    code: i32,
    data: ResponseValue,
    message: String,
    ttl: u8,
}

impl Display for ResponseData{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
enum ResponseValue {
    Login(LoginInfo),
    Value(serde_json::Value),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginInfo {
    cookie_info: serde_json::Value,
    message: String,
    sso: Vec<String>,
    status: u8,
    token_info: TokenInfo,
    url: String,
    #[serde(skip)]
    client: reqwest::Client,
}

// const APP_KEY: &str = "ae57252b0c09105d";
// const APPSEC: &str = "c75875c596a69eb55bd119e74b07cfe3";
const APP_KEY: &str = "783bbb7264451d82";
const APPSEC: &str = "2653583c8873dea268ab9386918b1d65";

#[derive(Debug)]
pub struct Client{
    client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Referer", header::HeaderValue::from_static("https://www.bilibili.com/"));
        headers.insert("Connection", header::HeaderValue::from_static("keep-alive"));
        let cookie_store = cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        Client{
            client: reqwest::Client::builder()
                .cookie_provider(std::sync::Arc::clone(&cookie_store))
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/63.0.3239.108")
                .default_headers(headers)
                .timeout(Duration::new(60, 0))
                .build()
                .unwrap(),
            cookie_store,
        }
    }

    pub async fn login(mut self, username: &str, password: &str) -> Result<(Self, LoginInfo)> {
        let result = match std::fs::File::open("cookies.json") {
            Ok(file) => {
                match self.login_by_cookies(file).await {
                    r @ Ok(_) => r,
                    Err(e) => {
                        println!("{:?}", e);
                        self.login_by_password(username, password).await
                    }
                }
            }
            Err(_) => Ok(self.login_by_password(username, password).await?)
        }?;
        Ok((self, result))
    }

    pub async fn login_by_cookies(&mut self, file: std::fs::File) -> Result<LoginInfo> {
        // Load an existing set of cookies, serialized as json
        // let mut file = std::fs::File::open("cookies.json")
        //     .map(std::io::BufReader::new)
        //     .unwrap();
        let login_info: LoginInfo = serde_json::from_reader(std::io::BufReader::new(file))?;
        self.set_cookie(&login_info.cookie_info);
        let response : ResponseData = self.client.get("https://api.bilibili.com/x/web-interface/nav")
            .send().await?
            .json().await?;
        println!("通过cookie登录");
        if response.code == 0 {
            Ok(login_info)
        } else {
            bail!("{}", response)
        }
    }

    pub async fn login_by_password(&mut self, username: &str, password: &str) -> Result<LoginInfo> {
        // The type of `payload` is `serde_json::Value`
        let (key_hash, pub_key) = self.get_key().await.unwrap();
        let pub_key = RsaPublicKey::from_public_key_pem(&pub_key).unwrap();
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let enc_data = pub_key.encrypt(&mut OsRng, padding, format!("{}{}", key_hash, password).as_bytes()).expect("failed to encrypt");
        let encrypt_password = encode(enc_data);
        let mut payload = json!({
            "actionKey": "appkey",
            "appkey": APP_KEY,
            "build": 6270200,
            "captcha": "",
            "challenge": "",
            "channel": "bili",
            "device": "phone",
            "mobi_app": "android",
            "password": encrypt_password,
            "permission": "ALL",
            "platform": "android",
            "seccode": "",
            "subid": 1,
            "ts": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "username": username,
            "validate": "",
        });
        let urlencoded = form_urlencoded::Serializer::new(String::new()).extend_pairs(payload.as_object().unwrap().iter().map(|(k, v)| {
            match v {
                Value::Number(x) => (k, x.to_string()),
                Value::String(x) => (k, x.into()),
                _ => panic!("未知")
            }
        })).finish();
        let sign = Client::sign(&urlencoded);
        payload["sign"] = Value::from(sign);
        let response: ResponseData = self.client.post("https://passport.bilibili.com/x/passport-login/oauth2/login")
            .form(&payload)
            .send().await?
            .json().await?;
        println!("通过密码登录");
        let result: LoginInfo = match &response.data {
            ResponseValue::Login(LoginInfo { cookie_info, .. }) if !cookie_info.is_null() => {
                let mut writer = File::create("cookies.json").await?;
                let login_info = response.data;
                writer.write(serde_json::to_string(&login_info)?.as_ref()).await?;
                // self.login_info = Some(response.clone().data.into());
                Ok(login_info.into())
            }
            _ => Err(anyhow!("{}", response))
        }?;
        self.set_cookie(&result.cookie_info);
        Ok(result)
    }

    pub async fn get_key(&self) -> Result<(String, String)> {
        let payload = json!({
            "appkey": APP_KEY,
            "sign": Client::sign(&format!("appkey={}", APP_KEY)),
        });
        let response: serde_json::Value = self.client.get("https://passport.bilibili.com/x/passport-login/web/key")
            .json(&payload)
            .send().await?
            .json().await?;
        Ok((response["data"]["hash"].as_str().unwrap().to_string(), response["data"]["key"].as_str().unwrap().to_string()))
    }

    pub fn sign(param: &str) -> String {
        let mut hasher = Md5::new();
        // process input message
        hasher.update(format!("{}{}", param, APPSEC));
        // acquire hash digest in the form of GenericArray,
        // which in this case is equivalent to [u8; 16]
        format!("{:x}", hasher.finalize())
    }

    fn set_cookie(&self, cookie_info: &serde_json::Value) {
        let mut store = self.cookie_store.lock().unwrap();
        for cookie in cookie_info["cookies"].as_array().unwrap() {
            let cookie = Cookie::build(cookie["name"].as_str().unwrap(), cookie["value"].as_str().unwrap())
                .domain("bilibili.com")
                .finish();
            store.insert_raw(&cookie, &Url::parse("https://bilibili.com/").unwrap());
        }
    }
}

impl From<ResponseValue> for LoginInfo {
    fn from(res: ResponseValue) -> Self {
        match res {
            ResponseValue::Login(v) => v,
            ResponseValue::Value(_) => panic!("错误调用")
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct TokenInfo {
    access_token: String,
    expires_in: u32,
    mid: u32,
    refresh_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Probe {
    #[serde(rename = "OK")]
    ok: u8,
    lines: Vec<Line>,
    probe: serde_json::Value,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Line {
    os: String,
    probe_url: String,
    query: String,
    #[serde(skip)]
    cost: u128
}

impl Probe {
    pub async fn probe() -> Result<Line> {
        let res: Self = reqwest::get("https://member.bilibili.com/preupload?r=probe").await?
            .json().await?;
        let client = if !res.probe["get"].is_null() {
            |url| reqwest::Client::new().get(url)
        } else {
            |url| reqwest::Client::new().post(url).body(vec![0; (1024. * 0.1 * 1024.) as usize])
        };
        let choice_line: Line = Default::default();
        for mut line in res.lines {
            let instant = Instant::now();
            if client(format!("https:{}", line.probe_url)).send().await?.status() == 200 {
                line.cost = instant.elapsed().as_millis();
                println!("{}: {}", line.query, line.cost);
                // if choice_line.cost > line.cost {
                //     choice_line = line
                // }
            };
        }
        Ok(choice_line)
    }
}

impl Default for Line {
    fn default() -> Self {
        Line{
            os: "upos".to_string(),
            probe_url: "//upos-sz-upcdnbda2.bilivideo.com/OK".to_string(),
            query: "upcdn=bda2&probe_version=20200810".to_string(),
            cost: u128::MAX
        }
    }
}

fn read_chunk(mut file: File, len: usize) -> impl Stream<Item=Result<Bytes>> {
    let mut buffer = vec![0u8; len];

    let mut buf = BytesMut::with_capacity(len);
    try_stream! {
        loop {
            let n = file.read(&mut buffer).await?;
            buf.put_slice(&buffer[..n]);
        // println!("{:?}", buf);
            if n == 0 {
                return;
            }
            yield buf.split().freeze();
        }
    }
}

