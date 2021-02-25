use std::error::Error;

use simledger;

mod runner;
mod macros;
mod util;

#[tokio::main]
async fn main() {
    let args = std::env::args();

    if args.len() != 2 {
        println!(
            "{} v{} -- insert coin to continue\n",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );

        println!(
            "Usage: {} ./filepath.csv",
            env!("CARGO_PKG_NAME"),
        );

        return;
    }

    let last_arg = args.last().unwrap();

    match runner::Runner::ignition(last_arg).await {
        Err(err) => eprintln!("Error: {:?}", err),
        Ok(_) => {}
    };
}
