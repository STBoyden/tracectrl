#![allow(dead_code)]

use std::str::FromStr;

use anyhow::{Error, Result};
use axum::http::Uri as AxumUri;
use reqwest::Url as ReqwestUrl;

pub enum UrlType {
    Axum(AxumUri),
    Reqwest(ReqwestUrl),
}

impl UrlType {
    pub fn to_reqwest(&self) -> Result<ReqwestUrl> {
        match self {
            Self::Axum(uri) => {
                ReqwestUrl::from_str(&format!("http://localhost:8080{}", uri)).map_err(Error::from)
            }
            Self::Reqwest(url) => Ok(url.clone()),
        }
    }

    pub fn to_axum(&self) -> Result<AxumUri> {
        match self {
            Self::Axum(uri) => Ok(uri.clone()),
            Self::Reqwest(url) => AxumUri::from_str(url.as_ref()).map_err(Error::from),
        }
    }
}
