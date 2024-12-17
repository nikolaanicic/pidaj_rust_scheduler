use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::SeedableRng;
use rand_pcg::Pcg64;

mod api;
mod client;
mod common;

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u64;

    let rng = Rc::new(RefCell::new(Pcg64::seed_from_u64(seed)));

    let mut api = api::API::new(5, rng.clone());
    let cl = client::Client::new(1, 10, rng.clone());

    cl.get(1, |r| api.get(r));

    // println!("{}", response);
}
