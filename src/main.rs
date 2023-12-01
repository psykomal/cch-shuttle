use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn handler(Path(nums): Path<String>) -> Result<String, StatusCode> {
    let nums_list = nums
        .split('/')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x: &str| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut s = nums_list.iter().fold(0, |acc, x| acc ^ x);

    s = s.pow(3);

    println!("{} {:?} {}", nums, nums_list, s);

    Ok(s.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/1/*nums", get(handler));

    Ok(router.into())
}
