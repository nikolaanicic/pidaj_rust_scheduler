use std::{env, sync::Arc};

use client::Client;
use futures::future::join_all;
use scheduler::Scheduler;

mod api;
mod client;
mod common;
mod scheduler;

async fn simulate(client_calls: i32, client_retry: i32) {
    let api = Arc::new(api::API::new(5));
    let scheduler = Arc::new(Scheduler::new(5));
    scheduler.run();
    let mut tasks = vec![];

    for i in 0..client_calls {
        tasks.push(Client::new(client_retry, Arc::clone(&api), Arc::clone(&scheduler)).get(i));
    }

    join_all(tasks).await;
    scheduler.shutdown().await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let client_calls: i32 = args
        .get(1)
        .expect("pass the client calls number")
        .parse::<i32>()
        .expect("pass a valid number");

    let client_retry: i32 = args
        .get(2)
        .expect("pass the client retry number")
        .parse::<i32>()
        .expect("pass a valid number");

    println!("{}_client_calls_{}_retry_ms", client_calls, client_retry);
    simulate(client_calls, client_retry).await;
}
