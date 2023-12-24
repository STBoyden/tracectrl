use std::borrow::Cow;

use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{request::Parts, StatusCode},
	Extension,
};
use sqlx::PgPool;

#[derive(Debug, Clone, Copy)]
pub struct ClientId(pub i32);

#[async_trait]
impl<S> FromRequestParts<S> for ClientId
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, Cow<'static, str>);

	async fn from_request_parts(
		parts: &mut Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		let Extension(pool) = Extension::<PgPool>::from_request_parts(parts, state)
			.await
			.map_err(|err| (err.status(), err.body_text().into()))?;

		if let Some(client_id) = parts.headers.get("client-id") {
			let id = client_id
				.to_str()
				.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string().into()))?
				.parse::<i32>()
				.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string().into()))?;

			if sqlx::query!(r#"SELECT id FROM "Clients" WHERE id = $1"#, id)
				.fetch_optional(&pool)
				.await
				.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string().into()))?
				.is_some()
			{
				Ok(ClientId(id))
			} else {
				Err((
					StatusCode::BAD_REQUEST,
					"`client-id` is missing or invalid".into(),
				))
			}
		} else {
			Err((
				StatusCode::BAD_REQUEST,
				"`client-id` header is missing or invalid".into(),
			))
		}
	}
}
