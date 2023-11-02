use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(
	title = "TraceCTRL",
	description = "API documentation for the TraceCTRL REST server"
))]
pub struct ApiDoc;
