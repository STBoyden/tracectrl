use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Uuid(pub uuid::Uuid);
