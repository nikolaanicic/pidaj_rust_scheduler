use std::{
    sync::Arc,
    time::{self, Instant},
};

use tokio::time::sleep;

use crate::{
    api::API,
    common::{Request, Response, StatusCode},
};

pub struct Client {
    client_retry_ms: i32,
    api: Arc<API>,
}

impl Client {
    pub fn new(client_retry_ms: i32, api: Arc<API>) -> Client {
        Client {
            client_retry_ms: client_retry_ms,
            api: api,
        }
    }

    fn get_retry_time(&self) -> f32 {
        self.client_retry_ms as f32 / 1000.0
    }

    pub async fn get(&self, id: i32) -> Response {
        let request = Request::new(id);
        let retry_time = self.get_retry_time();
        let start = Instant::now();

        let mut response = self.api.get(&request).await;

        while response.get_status() != StatusCode::OK {
            sleep(time::Duration::from_secs_f32(retry_time)).await;
            response = self.api.get(&request).await;
        }

        println!("{}", start.elapsed().as_secs_f32());
        response
    }
}
