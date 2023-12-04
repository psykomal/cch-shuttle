use axum::Router;

mod day0;
mod day1;
mod day4;

pub(crate) fn router() -> Router {
    Router::new()
        .nest("/", day0::router())
        .nest("/", day1::router())
        .nest("/", day4::router())
}
