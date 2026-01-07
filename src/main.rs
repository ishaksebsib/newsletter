use std::net::TcpListener;

use newsletter::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = newsletter::config::get_configuration().expect("Failed to load configuration");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    println!("Listening on http://{}", listener.local_addr()?);
    run(listener)?.await
}
