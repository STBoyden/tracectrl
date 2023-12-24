use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct RegisterClientResponse {
	pub client_id: i32,
}

#[utoipa::path(
	post,
	path="/api/get_or_register_client",
	responses(
		(status=200, body=RegisterClientResponse, description="The new ID."),
	),
)]
#[axum_macros::debug_handler]
pub async fn new_client(pool: Extension<PgPool>) -> Result<Json<RegisterClientResponse>> {
	register_client(pool, Path(0)).await
}

#[utoipa::path(
	post,
	path="/api/get_or_register_client/{id}",
	responses(
		(status=200, body=RegisterClientResponse, description="The new or existing client ID. If the ID is new, it is highly unlikely that this id will be the same as the one that is returned."),
	),
	params(
		("id" = i32, Path, description = "New or existing client ID")
	)
)]
#[axum_macros::debug_handler]
pub async fn register_client(
	Extension(pool): Extension<PgPool>,
	Path(id): Path<i32>,
) -> Result<Json<RegisterClientResponse>> {
	if let Some(record) = sqlx::query!(r#"SELECT * FROM "Clients" WHERE id = $1"#, id)
		.fetch_optional(&pool)
		.await?
	{
		sqlx::query!(
			r###"
			UPDATE "Clients"
			SET last_connected = now()
			WHERE id = $1
		"###,
			record.id
		)
		.execute(&pool)
		.await?;

		return Ok(Json(RegisterClientResponse {
			client_id: record.id,
		}));
	}

	let record = sqlx::query!(r#"INSERT INTO "Clients" DEFAULT VALUES RETURNING id"#)
		.fetch_one(&pool)
		.await?;

	Ok(Json(RegisterClientResponse {
		client_id: record.id,
	}))
}
