#![allow(dead_code)]

use std::str::FromStr;

use crate::{prelude::*, utils::W};

use axum::http::Uri as AxumUri;
use reqwest::Url as ReqwestUrl;

impl TryFrom<W<&AxumUri>> for ReqwestUrl {
	type Error = Error;

	fn try_from(value: W<&AxumUri>) -> Result<ReqwestUrl> {
		ReqwestUrl::from_str(&format!("http://localhost:8080{}", *value)).map_err(Error::from)
	}
}

impl TryFrom<W<&ReqwestUrl>> for AxumUri {
	type Error = Error;

	fn try_from(value: W<&ReqwestUrl>) -> Result<AxumUri> {
		AxumUri::from_str(value.as_ref()).map_err(Error::from)
	}
}
