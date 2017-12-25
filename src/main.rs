extern crate structopt;
#[macro_use]
extern crate structopt_derive;


mod config;

fn main() {
    let config = config::get_config();
    println!("{:?}", config);


}
