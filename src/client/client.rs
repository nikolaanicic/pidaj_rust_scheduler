use std::{
    sync::Arc,
    time::{self, Duration, SystemTime, UNIX_EPOCH},
};

use tokio::time::sleep;

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

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => println!("{}:{}:-1:client",id, n.as_millis()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        self.scheduler
            .add_task(
                id,
                Box::new(move || {
                    let a = Arc::clone(&api);
                    Box::pin(async move { a.get(&Request::new(id)).await })
                }),
            )
            .await
            .notified()
            .await;

        let mut response = loop {
            if let Some(res) = self.scheduler.get_result(id).await {
                break res;
            }
        };
        let mut tries = 1;


        while response.get_status() != StatusCode::OK {
            sleep(time::Duration::from_secs_f32(retry_time)).await;
            let api = Arc::clone(&self.api);

            self.scheduler
                .add_task(
                    id,
                    Box::new(move || {
                        let a = Arc::clone(&api);
                        Box::pin(async move { a.get(&Request::new(id)).await })
                    }),
                )
                .await
                .notified()
                .await;

            response = loop {
                if let Some(res) = self.scheduler.get_result(id).await {
                    break res
                }

                sleep(Duration::from_millis(10)).await;
            };

            tries += 1;
        }
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => println!("{}:{}:{}:client",id, n.as_millis(),tries),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        response
    }
}
