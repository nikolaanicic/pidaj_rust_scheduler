use std::{env, sync::Arc, time::Duration};

use api::API;
use client::Client;
use common::Response;
use futures::future::join_all;
use rand::{thread_rng, Rng};
use scheduler::Scheduler;
use tokio::time::sleep;

mod api;
mod client;
mod common;
mod scheduler;

async fn sleep_client(api:Arc<API>, sch: Arc<Scheduler>, id:i32, client_retry: i32) -> Response{
            
    let duration = Duration::from_millis(thread_rng().gen_range(10..=100));
    sleep(duration).await;

    return Client::new(client_retry, Arc::clone(&api), Arc::clone(&sch)).get(id).await;
}

async fn simulate(client_calls: i32, client_retry: i32) {
    let api = Arc::new(api::API::new(5));
    let scheduler = Scheduler::new(5);
    scheduler.run();
    let mut tasks = vec![];

    for i in 0..client_calls {
        tasks.push(sleep_client(api.clone(), scheduler.clone(), i, client_retry));
    }

    join_all(tasks).await;
    scheduler.shutdown().await;
}


// #[tokio::main(flavor = "multi_thread")]
// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main(){
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
