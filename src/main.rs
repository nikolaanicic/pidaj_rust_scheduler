use std::{env, sync::Arc};

use futures::future::join_all;

mod api;
mod client;
mod common;

async fn simulate(client_calls: i32, client_retry: i32) {
    let api = Arc::new(api::API::new(5));
    let cl = Arc::new(client::Client::new(client_retry, Arc::clone(&api)));

    let mut tasks = vec![];

    for i in 0..client_calls {
        tasks.push(cl.get(i));
    }

    join_all(tasks).await;
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
