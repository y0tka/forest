use forest::{print_field, Cell};

use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use std::thread;
use std::time::Duration;

const API_URL: &str = "http://127.0.0.1:3030/v1";
const STEPS: usize = 100;
const DELAY: usize = 250;

fn main() {
    let client = Client::new();

    let mut field = get_random_field(&client, 10, 10, 10, 10).unwrap();

    for _ in 0..STEPS {
        field = get_simulation_step(&client, &field).unwrap();
        print_field(&field);
        thread::sleep(Duration::from_millis(DELAY as u64));
    }
}

fn get_random_field(
    client: &Client,
    size: usize,
    grass: usize,
    trees: usize,
    flames: usize,
) -> Result<Vec<Cell>, StatusCode> {
    let response = client
        .get(String::from(API_URL) + "/field/random")
        .header(CONTENT_TYPE, "application/json")
        .query(&[
            ("size", size),
            ("grass", grass),
            ("trees", trees),
            ("flames", flames),
        ])
        .send()
        .unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Vec<Cell>>() {
            Ok(v) => Ok(v),
            Err(e) => panic!("{}", e),
        },
        e => Err(e),
    }
}

fn get_simulation_step(client: &Client, field: &Vec<Cell>) -> Result<Vec<Cell>, StatusCode> {
    let response = client
        .get(String::from(API_URL) + "/simulation/step")
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&field).unwrap())
        .send()
        .unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Vec<Cell>>() {
            Ok(v) => Ok(v),
            Err(e) => panic!("{}", e),
        },
        e => Err(e),
    }
}
