mod asset;
mod banner;
mod cli;

use asset::Asset;
use banner::print_banner;
use cli::Opt;

use colour::{red, yellow};
use core::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::{Duration, Instant}};


fn main() {
    print_banner();

    let opts = Opt::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler( move || {
        println!("\nreceived Ctrl+C! Stopping program...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    let client = reqwest::blocking::Client::new();

    while running.load(Ordering::SeqCst) {
        let start = Instant::now();

        let resp = client.get(format!("{}/registries/{}/assets", opts.asset_domain(), opts.network_id())).send().expect("Error sending Asset request");
        if !resp.status().is_success() {
            match resp.status() {
                reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::NOT_FOUND => {
                    //Asset registry specific errors
                    if resp.headers()[reqwest::header::CONTENT_TYPE].to_str().unwrap().contains(&"json"){
                        panic!("Error requesting Assets. Error: {}.", resp.text().unwrap());
                    }
                    //All other errors
                    panic!("Error requesting Assets. Status Code: {}", resp.status());
                }
                reqwest::StatusCode::REQUEST_TIMEOUT => {
                    thread::sleep(Duration::from_millis(opts.delay()));
                    continue;
                }
                _ => panic!("Error requesting Assets. Status Code: {}", resp.status()),
            }
        }
        let assets: Vec<Asset> = match serde_json::from_str(&resp.text().unwrap()){
            Ok(value) => value,
            Err(err) => {
                yellow!("Error parsing JSON: {}", err);
                continue;
            },
        };

        let resp = client.get(format!("{}/admin/filters", opts.asset_domain())).basic_auth(opts.user(), Some(opts.password())).send().expect("Error requesting filters");
        if !resp.status().is_success() {
            match resp.status() {
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                    let status_code = resp.status();
                    if resp.text().unwrap().contains("Unauthorized") {
                        panic!("Error requesting filters. Are you using wrong credentials?");
                    }
                    panic!("Error requesting filters. Status Code: {}", status_code);
                }
                reqwest::StatusCode::REQUEST_TIMEOUT => {
                    thread::sleep(Duration::from_millis(opts.delay()));
                    continue;
                }
                _ => panic!("Error requesting filters. Status Code: {}", resp.status()),
            }
        }
        let bad_words = resp.text().unwrap().replace("\",\"", "\n").strip_prefix("[\"").unwrap().to_string().strip_suffix("\"]\n").unwrap().to_string();

        let total_count = assets.len();
        let mut remove_count = 0;

        for asset in assets {
            if asset.contains_bad_word(&bad_words){
                let _resp = client.delete(format!("{}/admin/{}/assets/byID/{}", opts.asset_domain(), opts.network_id(), asset.id())).basic_auth(opts.user(), Some(opts.password())).send().unwrap();
                remove_count += 1;
            }
        }

        let duration = start.elapsed();
        print!("Blue dragon found and removed ");
        if remove_count==0 {
            print!("{}", remove_count);
        } else {
            red!("{}", remove_count);
        }
        println!("/{} Asset(s) with bad words in {:?}", total_count, duration);

        thread::sleep(Duration::from_millis(opts.delay()));
    }
}