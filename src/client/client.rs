use rand::Rng;
use rand_pcg::Pcg64;
use std::{
    cell::RefCell,
    rc::Rc,
    thread::sleep,
    time::{self, Instant},
};

use crate::common::{Request, Response, StatusCode};

pub struct Client {
    min_retry_time: i32,
    max_retry_time: i32,
    random_generator: Rc<RefCell<Pcg64>>,
}

impl Client {
    pub fn new(
        min_retry_time: i32,
        max_retry_time: i32,
        random_generator: Rc<RefCell<Pcg64>>,
    ) -> Client {
        Client {
            min_retry_time: min_retry_time,
            max_retry_time: max_retry_time,
            random_generator: random_generator,
        }
    }

    fn get_retry_time(&self) -> f32 {
        let mut rng = self.random_generator.borrow_mut();

        return (rng.gen_range(self.min_retry_time..=self.max_retry_time) as f32)
            * (rng.gen::<f32>());
    }

    pub fn get<F>(&self, id: i32, ref mut api_get_stub_call: F) -> Response
    where
        F: FnMut(&Request) -> Response,
    {
        let request = Request::new(id);

        let start = Instant::now();
        let mut response = api_get_stub_call(&request);

        while response.get_status() != StatusCode::OK {
            sleep(time::Duration::from_secs_f32(self.get_retry_time()));
            response = api_get_stub_call(&request);
        }

        println!("{}", start.elapsed().as_millis());

        response
    }
}
