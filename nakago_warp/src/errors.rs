use std::convert::Infallible;

use serde::Serialize;
use warp::{http::StatusCode, reply, Rejection, Reply};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// Handle any expected Warp rejections
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found".to_string();
    } else {
        debug!("Unhandled Rejection: {:?}", err);

        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error".to_string();
    }

    let json = reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(reply::with_status(json, code))
}
