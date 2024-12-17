use std::{cell::RefCell, rc::Rc, thread::sleep, time};

use super::{ERR_PROB_FAULT, STATUS_SUCCESS};
use crate::{
    api::ERR_MAX_CONN,
    common::{get_err_response, get_ok_response, Request, Response},
};

use rand::Rng;
use rand_pcg::Pcg64;

#[derive(Debug)]
pub struct API {
    current_connections: i32,
    error_margin: f32,
    max_connections: i32,
    min_compute_time: f32,
    max_compute_time: f32,
    random_generator: Rc<RefCell<Pcg64>>,
}

impl API {
    pub fn new(max_conns: i32, random_generator: Rc<RefCell<Pcg64>>) -> API {
        API {
            current_connections: 0,
            error_margin: 0.2,
            max_connections: max_conns,
            min_compute_time: 1.0,
            max_compute_time: 3.0,
            random_generator: random_generator,
        }
    }

    fn get_current_conns(&self) -> i32 {
        self.current_connections
    }

    fn inc_conns(&mut self) {
        self.current_connections += 1;
    }

    fn dec_conns(&mut self) {
        self.current_connections -= 1;
    }

    fn compute_time(&self) -> f32 {
        let conns = std::cmp::max(self.get_current_conns(), 1);

        let min_time = self.min_compute_time;
        let max_time = self.max_compute_time;

        let mut rng = self.random_generator.borrow_mut();

        return (rng.gen_range(min_time..=max_time) as f32 * rng.gen::<f32>()) / 10.0
            * conns as f32;
    }
    fn compute_response(&self) -> Response {
        // await async.sleep(self.compute_time())

        sleep(time::Duration::from_secs_f32(self.compute_time()));

        if self.random_generator.borrow_mut().gen::<f32>() <= self.error_margin {
            get_err_response(ERR_PROB_FAULT.to_string())
        } else {
            get_ok_response(STATUS_SUCCESS.to_string())
        }
    }

    pub fn get(&mut self, _: &Request) -> Response {
        if self.get_current_conns() == self.max_connections {
            return get_err_response(ERR_MAX_CONN.to_string());
        }

        self.inc_conns();

        let response = self.compute_response();

        self.dec_conns();

        return response;
    }
}
