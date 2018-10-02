use errors::*;
use std::fmt;
use chrootable_https::{self, HttpClient, Body, Request, Uri};
use chrootable_https::http::request::Builder as RequestBuilder;
use chrootable_https::header::CONTENT_TYPE;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use sn0int_common::api::*;

pub const API_URL: &str = "http://[::1]:8000";


pub struct Client {
    server: String,
    client: chrootable_https::Client<chrootable_https::Resolver>,
    session: Option<String>,
}

impl Client {
    pub fn new<I: Into<String>>(server: I) -> Result<Client> {
        let client = chrootable_https::Client::with_system_resolver()?;
        Ok(Client {
            server: server.into(),
            client,
            session: None,
        })
    }

    pub fn authenticate<I: Into<String>>(&mut self, session: I) {
        self.session = Some(session.into());
    }

    pub fn random_session() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(32).collect()
    }

    pub fn request<T: DeserializeOwned + fmt::Debug>(&self, mut request: RequestBuilder, body: Body) -> Result<T> {
        if let Some(session) = &self.session {
            info!("Adding session token");
            request.header("Auth", session.as_str());
        }

        let request = request.body(body)?;

        let resp = self.client.request(request)?;
        info!("response: {:?}", resp);

        let reply = serde_json::from_slice::<ApiResponse<T>>(&resp.body)?;
        info!("api: {:?}", reply);
        let reply = reply.success()?;
        info!("api(success): {:?}", reply);

        Ok(reply)
    }

    pub fn get<T: DeserializeOwned + fmt::Debug>(&self, url: &str) -> Result<T> {
        let url = url.parse::<Uri>()?;

        info!("requesting: {:?}", url);
        let request = Request::get(url);
        self.request(request, Body::empty())
    }

    pub fn post<T, S>(&self, url: &str, body: &S) -> Result<T>
        where T: DeserializeOwned + fmt::Debug,
              S: Serialize + fmt::Debug,
    {
        let url = url.parse::<Uri>()?;

        info!("requesting: {:?}", url);
        let mut request = Request::post(url);
        request.header(CONTENT_TYPE, "application/json; charset=utf-8");
        let body = serde_json::to_string(body)?;
        self.request(request, body.into())
    }

    pub fn verify_session(&self) -> Result<String> {
        let url = format!("{}/api/v0/whoami", self.server);
        let resp = self.get::<WhoamiResponse>(&url)?;
        Ok(resp.user)
    }

    pub fn publish_module(&self, name: &str, body: String) -> Result<PublishResponse> {
        let url = format!("{}/api/v0/publish/{}", self.server, name);
        let reply = self.post::<PublishResponse, _>(&url, &PublishRequest {
            code: body.into(),
        })?;
        Ok(reply)
    }

    pub fn download_module(&self, module: &str, version: &str) -> Result<DownloadResponse> {
        let url = format!("{}/api/v0/dl/{}/{}", self.server, module, version);
        let reply = self.get::<DownloadResponse>(&url)?;
        Ok(reply)
    }

    pub fn query_module(&self, module: &str) -> Result<ModuleInfoResponse> {
        let url = format!("{}/api/v0/info/{}", self.server, module);
        let reply = self.get::<ModuleInfoResponse>(&url)?;
        Ok(reply)
    }
}