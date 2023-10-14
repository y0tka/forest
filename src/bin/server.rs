use std::collections::HashMap;

use warp::Filter;

use forest::{get_empty_field, get_field_step, get_random_field, Cell};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://0.0.0.0:8080")
        .allow_header("Content-Type")
        .allow_methods(vec!["GET", "POST"]);

    let simstep = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("simulation"))
        .and(warp::path("step"))
        .and(warp::body::json())
        .map(|mut field: Vec<Cell>| {
            field = get_field_step(&field);
            warp::reply::json(&field)
        })
        .with(cors.clone());

    let fieldgen = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("field"))
        .and(warp::path("empty"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("size") {
            Some(size) => match size.parse::<usize>() {
                Ok(v) => warp::reply::with_status(
                    warp::reply::json(&get_empty_field(v)),
                    warp::http::StatusCode::OK,
                ),
                Err(_) => warp::reply::with_status(
                    warp::reply::json(&""),
                    warp::http::StatusCode::BAD_REQUEST,
                ),
            },
            None => warp::reply::with_status(
                warp::reply::json(&""),
                warp::http::StatusCode::BAD_REQUEST,
            ),
        })
        .with(cors.clone());

    let randomfield = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("field"))
        .and(warp::path("random"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let mut size: usize = 0;
            let mut grass: usize = 0;
            let mut trees: usize = 0;
            let mut flames: usize = 0;
            let mut err = false;
            match p.get("size") {
                Some(v) => match v.parse::<usize>() {
                    Ok(parsed) => size = parsed,
                    _ => err = true,
                },
                _ => err = true,
            }
            match p.get("grass") {
                Some(v) => match v.parse::<usize>() {
                    Ok(parsed) => grass = parsed,
                    _ => err = true,
                },
                _ => (),
            }
            match p.get("trees") {
                Some(v) => match v.parse::<usize>() {
                    Ok(parsed) => trees = parsed,
                    _ => err = true,
                },
                _ => (),
            }
            match p.get("flames") {
                Some(v) => match v.parse::<usize>() {
                    Ok(parsed) => flames = parsed,
                    _ => err = true,
                },
                _ => (),
            }
            if err || (flames + grass + trees > size * size) {
                return warp::reply::with_status(
                    warp::reply::json(&""),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            }
            return warp::reply::with_status(
                warp::reply::json(&get_random_field(size, grass, trees, flames)),
                warp::http::StatusCode::OK,
            );
        })
        .with(cors.clone());

    warp::serve(simstep.or(fieldgen).or(randomfield))
        .run(([127, 0, 0, 1], 3030))
        .await
}
