#[macro_use] extern crate rocket;
use rand::Rng;
use rocket::serde::json::{Json};

const MAX_LINES:u32 = 6802;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/words/<n>")]
fn words(n: usize) -> Json<Vec<String>> {
    let mut numbers: Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();
    // let mut words: Vec<String> = Vec::new();

    for _ in 0..n {
        let random_number: usize = rng.gen_range(1..MAX_LINES).try_into().unwrap();
        numbers.push(random_number);
    }

    let words: Vec<String> = match get_words("./nounlist.txt", &numbers) {
        Ok(words) => words,
        Err(_) => vec!["Error".to_string()]
    };
    Json(words)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, words])
}

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn get_words(path: &str, indices: &Vec<usize>) -> Result<Vec<String>, Error> {

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines = Vec::new();


    for (index, line) in buffered.lines().enumerate() {
        if indices.contains(&index) {
            lines.push(line?);
        }
    }

    Ok(lines)
}