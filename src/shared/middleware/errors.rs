
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use log::error;
use crate::shared::middleware::response::response_error;

pub fn add_error_header<B>(mut res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json; charset=utf-8"),
    );

    let mut error_msg: String = match res.response().error() {
        Some(e) => format!("{}", e.to_string()),
        None => String::from("Unknown Error")
    };

    if res.response().status().is_server_error() {
        error!("{}", error_msg);
        error_msg = String::from("Internal Server Error");
    }

    let error_res = response_error(error_msg.as_str());
    let error_res = serde_json::to_string(&error_res)?;

    let (req, res) = res.into_parts();
    let res = res.set_body(error_res);
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
