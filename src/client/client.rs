use std::{
    sync::Arc,
    time::{self},
};

use tokio::{
    task,
    time::{sleep, Instant},
};

use crate::{
    api::API,
    common::{Request, Response, StatusCode},
    scheduler::Scheduler,
};

pub struct Client {
    client_retry_ms: i32,
    api: Arc<API>,
    scheduler: Arc<Scheduler>,
}

impl Client {
    pub fn new(client_retry_ms: i32, api: Arc<API>, scheduler: Arc<Scheduler>) -> Client {
        Client {
            client_retry_ms: client_retry_ms,
            api: api,
            scheduler: scheduler,
        }
    }

    fn get_retry_time(&self) -> f32 {
        self.client_retry_ms as f32 / 1000.0
    }

    pub async fn get(self, id: i32) -> Response {
        let retry_time = self.get_retry_time();
        let api = Arc::clone(&self.api);

        let start = Instant::now();

        self.scheduler
            .add_task(
                id,
                task::spawn(async move { api.get(&Request::new(id)).await }),
            )
            .await
            .notified()
            .await;

        let mut response = loop {
            if let Some(res) = self.scheduler.get_result(id).await {
                break res;
            }
        };

        while response.get_status() != StatusCode::OK {
            sleep(time::Duration::from_secs_f32(retry_time)).await;
            let api = Arc::clone(&self.api);

            self.scheduler
                .add_task(
                    id,
                    task::spawn(async move { api.get(&Request::new(id)).await }),
                )
                .await
                .notified()
                .await;

            response = loop {
                if let Some(res) = self.scheduler.get_result(id).await {
                    break res;
                }
            };
        }
        println!("{}: {}", response, start.elapsed().as_secs_f32());
        response
    }
}
