use std::convert::Infallible;

use axum::response::IntoResponse;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("generic {0}")]
	Generic(String),
	#[error("status code {0}, error: {1}")]
	ResponseError(StatusCode, String),

	#[error(transparent)]
	AxumError(#[from] axum::http::Error),
	#[error(transparent)]
	AxumUriError(#[from] axum::http::uri::InvalidUri),
	#[error(transparent)]
	ReqwestError(#[from] reqwest::Error),
	#[error(transparent)]
	UrlError(#[from] url::ParseError),
	#[error(transparent)]
	Infallible(#[from] Infallible),
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			Error::Generic(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
			Error::ResponseError(status_code, message) => (status_code, message),
			Error::UrlError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
			Error::AxumError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
			Error::AxumUriError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
			Error::ReqwestError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
			Error::Infallible(_) => unreachable!(),
		}
		.into_response()
	}
}
