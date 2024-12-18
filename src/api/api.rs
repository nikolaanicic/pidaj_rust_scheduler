use super::{ERR_PROB_FAULT, STATUS_SUCCESS};
use crate::{
    api::ERR_MAX_CONN,
    common::{get_err_response, get_ok_response, Request, Response},
};
use std::{cmp::Ordering, sync::Arc, time};
use tokio::time::sleep;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct API {
    current_connections: Arc<Mutex<i32>>,
    error_margin: f32,
    max_connections: i32,
    min_compute_time: f32,
    max_compute_time: f32,
    random_generator: Arc<Mutex<Pcg64>>,
}

impl API {
    pub fn new(max_conns: i32) -> API {
        API {
            current_connections: Arc::new(Mutex::new(0)),
            error_margin: 0.2,
            max_connections: max_conns,
            min_compute_time: 50.0,
            max_compute_time: 800.0,
            random_generator: Arc::new(Mutex::new(Pcg64::from_entropy())),
        }
    }

    async fn compute_time(&self, active_conns: i32) -> f32 {
        let min_time = self.min_compute_time;
        let max_time = self.max_compute_time;
        let mut rng = self.random_generator.lock().await;

        return (rng.gen_range(min_time..=max_time) as f32 * rng.gen::<f32>() / 100.0) / 10.0
            * std::cmp::max(active_conns, 1) as f32;
    }

    async fn compute_response(&self, active_conns: i32) -> Response {
        let time = self.compute_time(active_conns).await;
        sleep(time::Duration::from_secs_f32(time)).await;

        let mut rng = self.random_generator.lock().await;
        let cmp = rng.gen::<f32>().total_cmp(&self.error_margin);

        if cmp == Ordering::Equal || cmp == Ordering::Less {
            get_err_response(ERR_PROB_FAULT.to_string(), active_conns)
        } else {
            get_ok_response(STATUS_SUCCESS.to_string(), active_conns)
        }
    }

    pub async fn get(&self, _: &Request) -> Response {
        let conns;
        {
            let mut active_conns = self.current_connections.lock().await;
            if *active_conns == self.max_connections {
                return get_err_response(ERR_MAX_CONN.to_string(), *active_conns);
            }

            *active_conns += 1;
            conns = *active_conns;
        }

        let response = self.compute_response(conns).await;

        let mut active_conns = self.current_connections.lock().await;
        *active_conns -= 1;

        return response;
    }
}
