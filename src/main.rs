#![feature(async_closure)]

mod analyzer;
mod app;
mod fingerprint;

#[async_std::main]
async fn main() {
    let app = app::main();

    match app {
        Ok(_) => {
            println!("App ran succesfully");
        }
        Err(e) => {
            println!("App had a problem...");
            println!("{}", e);
        }
    }
}
