use axum::Router;

mod day0;
mod day1;

pub(crate) fn router() -> Router {
    Router::new()
        .nest("/", day0::router())
        .nest("/", day1::router())
}
