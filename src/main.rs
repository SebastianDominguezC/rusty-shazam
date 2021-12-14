#![feature(async_closure)]

mod app;
mod fingerprint;
#[tokio::main]
async fn main() {
    app::main();
}
