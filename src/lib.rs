extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Theme {
    theme_id: u64,
    choices: Vec<String>,
    dr_class: u64,
}

#[derive(Deserialize, Clone)]
struct Vote {
    theme_id: u64,
    answer: u64,
    created_at: u64,
    expired_at: u64,
}

#[derive(Deserialize)]
struct Request {
    themes: Vec<Theme>,
    votes: Vec<Vote>,
}

#[derive(Serialize)]
struct Transition {
    timestamp: u64,
    percentage: Vec<f64>,
}

#[derive(Serialize)]
struct Response {
    theme_id: u64,
    percentage: Vec<f64>,
    short_transition: Vec<Transition>,
    long_transition: Vec<Transition>,
}

fn get_melting_rate(dr_class: u64) -> u64 {
    match dr_class {
        1 => 2400 * 1000,
        2 => 7200 * 1000,
        3 => 21600 * 1000,
        4 => 64800 * 1000,
        _ => 194400 * 1000,
    }
}

fn get_result_interval(dr_class: u64) -> u64 {
    match dr_class {
        1 => 400 * 1000,
        2 => 20 * 60 * 1000,
        3 => 60 * 60 * 1000,
        4 => 3 * 60 * 60 * 1000,
        _ => 9 * 60 * 60 * 1000,
    }
}

fn eval_formula(elapsed: u64, melting_rate: u64) -> f64 {
    let val = elapsed / melting_rate;
    (4 * val + 5) as f64 / (val * val + 4 * val + 5) as f64
}

fn calc_result(now: u64, melting_rate: u64, num_of_choices: u64, votes: &Vec<Vote>) -> Vec<f64> {
    let mut points: Vec<f64> = vec![0.0; num_of_choices as usize];

    votes
        .iter()
        .filter(|vote| vote.created_at <= now && (vote.expired_at == 0 || vote.expired_at > now))
        .for_each(|vote| {
            points[vote.answer as usize] += eval_formula(now - vote.created_at, melting_rate)
        });

    let sum: f64 = points.iter().sum();
    points
        .iter()
        .map(|point| (point / sum * 1000000.0).round() / 10000.0)
        .collect()
}

fn calc_transition(
    now: u64,
    theme: &Theme,
    votes: &Vec<Vote>,
) -> (Vec<Transition>, Vec<Transition>) {
    let result_interval = get_result_interval(theme.dr_class);
    let melting_rate = get_melting_rate(theme.dr_class);
    let cur_votes = votes
        .iter()
        .filter(|vote| vote.theme_id == theme.theme_id)
        .map(|vote| vote.clone())
        .collect();
    let callback = |i| {
        let timestamp = now - i * result_interval;
        Transition {
            timestamp,
            percentage: calc_result(
                timestamp,
                melting_rate,
                theme.choices.len() as u64,
                &cur_votes,
            ),
        }
    };

    (
        (0..60).map(callback).collect(),
        (0..60).map(|i| i * 24).map(callback).collect(),
    )
}

#[wasm_bindgen]
pub fn computer(request: &str, now: f64) -> String {
    let request: Request = serde_json::from_str(request).unwrap();

    let response: Vec<Response> = request
        .themes
        .iter()
        .map(|theme| {
            let (short_transition, long_transition) =
                calc_transition(now as u64, theme, &request.votes);
            Response {
                theme_id: theme.theme_id,
                percentage: short_transition[0].percentage.clone(),
                short_transition,
                long_transition,
            }
        })
        .collect();

    json!(response).to_string()
}
