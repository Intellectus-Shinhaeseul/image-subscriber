use std::path::PathBuf;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    zenoh::init_log_from_env_or("debug");

    let config_file = "zenoh.json5";
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push(config_file);
    let config = zenoh::Config::from_file(config_path).expect("Failed to load configuration file");

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Declare Subscriber on 'camera1~4'...");
    let sub1 = session.declare_subscriber("camera1").await.unwrap();
    let sub2 = session.declare_subscriber("camera2").await.unwrap();
    let sub3 = session.declare_subscriber("camera3").await.unwrap();
    let sub4 = session.declare_subscriber("camera4").await.unwrap();

    tokio::spawn(process_subscriber(sub1));
    tokio::spawn(process_subscriber(sub2));
    tokio::spawn(process_subscriber(sub3));
    tokio::spawn(process_subscriber(sub4));

    println!("Press CTRL-C to quit...");
    loop {
        sleep(Duration::from_millis(1)).await;
    }
}

async fn process_subscriber(
    subscriber: zenoh::pubsub::Subscriber<
        zenoh::handlers::FifoChannelHandler<zenoh::sample::Sample>,
    >,
) {
    let keyexpr = subscriber.key_expr().to_string();

    loop {
        match tokio::time::timeout(std::time::Duration::from_secs(5), subscriber.recv_async()).await
        {
            Ok(Ok(_sample)) => {
                // Do nothing
            }
            Ok(Err(e)) => {
                eprintln!("Message reception error for keyexpr {}: {:?}", keyexpr, e);
            }
            Err(e) => {
                eprintln!("Message reception timeout for keyexpr {}: {:?}", keyexpr, e);
            }
        }
    }
}
