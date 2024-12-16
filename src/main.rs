use common::Request;

mod api;
mod common;

fn main() {
    let mut api = api::API::new(5);

    println!("{:?}", api);

    let request = Request::new(1);

    println!("{}", api.get(request));
}
