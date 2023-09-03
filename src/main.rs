use newsruster::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let message = &format!("Healt check url: http://127.0.0.1:{}/health_check", &port);

    println!("{message}");

    let _server = run(listener).expect("Failed to bind address").await;

    Ok(())
}
