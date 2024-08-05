use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening on http://{}", address.local_addr()?);
    run(address)?.await
}
