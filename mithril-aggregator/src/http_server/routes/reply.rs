use mithril_common::entities::InternalServerError;
use serde::Serialize;
use warp::http::StatusCode;

pub fn json<T>(value: &T, status_code: StatusCode) -> Box<dyn warp::Reply>
where
    T: Serialize,
{
    Box::new(warp::reply::with_status(
        warp::reply::json(value),
        status_code,
    ))
}

pub fn empty(status_code: StatusCode) -> Box<dyn warp::Reply> {
    Box::new(warp::reply::with_status(warp::reply::reply(), status_code))
}

pub fn internal_server_error(message: String) -> Box<dyn warp::Reply> {
    json(
        &InternalServerError::new(message),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}
