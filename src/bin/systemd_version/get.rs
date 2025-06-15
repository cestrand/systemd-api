use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use systemd_api::version::{systemctl_version, Version};

#[derive(Debug)]
pub enum Error {
    Internal
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: &'static str,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Error::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't get version"),
        };
        let body = Json(ErrorResponse {error: msg });
        (status, body).into_response()
    }
}

#[utoipa::path(
    get,
    path = "/version",
    responses(
        (status = 200, description = "Found version successfully", body = Version),
        (status = 500, description = "Error")
    )
)]
pub async fn handler() -> Result<impl IntoResponse, Error> {
    let version = systemctl_version().map_err(|_| Error::Internal)?;
    Ok(Json(version))
}
