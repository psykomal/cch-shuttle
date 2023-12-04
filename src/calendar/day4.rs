use axum::response::IntoResponse;
use axum::{
    debug_handler,
    extract::Json,
    extract::Path,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
}

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i64,
}

async fn strength(Json(payload): Json<Vec<Reindeer>>) -> String {
    payload
        .iter()
        .map(|player| player.strength)
        .sum::<i64>()
        .to_string()
}

#[derive(Deserialize, Debug)]
struct ContestInput {
    name: String,
    strength: i64,
    speed: f64,
    height: i64,
    antler_width: i64,
    snow_magic_power: i64,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten: i64,
}

#[derive(Serialize, Clone, Debug, Default)]
struct ContestOutput {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl IntoResponse for ContestOutput {
    fn into_response(self) -> axum::response::Response {
        let json = serde_json::to_string(&self).unwrap();
        axum::response::Response::builder()
            .header("content-type", "application/json")
            .body(json)
            .unwrap()
            .into_response()
    }
}

#[debug_handler]
async fn contest(Json(payload): Json<Vec<ContestInput>>) -> ContestOutput {
    println!("{:?}", payload);
    let fastest = payload
        .iter()
        .enumerate()
        .map(|(i, player)| (i, player.speed))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    let tallest = payload
        .iter()
        .enumerate()
        .map(|(i, player)| (i, player.height))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    let magician = payload
        .iter()
        .enumerate()
        .map(|(i, player)| (i, player.snow_magic_power))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    let candies_eaten = payload
        .iter()
        .enumerate()
        .map(|(i, player)| (i, player.candies_eaten))
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    ContestOutput {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            payload[fastest].strength, payload[fastest].name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            payload[tallest].name, payload[tallest].antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            payload[magician].name, payload[magician].snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            payload[candies_eaten].name, payload[candies_eaten].favorite_food
        ),
    }
}
