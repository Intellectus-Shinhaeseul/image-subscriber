# Image Subscriber Package

This Rust package is designed to subscribe to **four topics (camera1 to camera4)** simultaneously using the `zenoh` protocol.

## Features
- Subscribes to multiple topics: `camera1`, `camera2`, `camera3`, and `camera4`.
- Displays a timeout message in the console if no message is received for more than 5 seconds.

## How It Works
1. Ensure the Zenoh router is running using the `router.json5` configuration file from the `image-publisher` package:
   ```bash
   zenohd -c router.json5
   ```
2. Run the package using `cargo run`.
3. The subscriber will automatically start listening to the topics `camera1`, `camera2`, `camera3`, and `camera4`.

4. If no message is received for more than 5 seconds, a timeout message will be displayed in the console.

## Example Usage
```bash
$ cargo run
Declare Subscriber on 'camera1~4'...
Message reception timeout for keyexpr camera1: Elapsed(())
```

## Dependencies
This project uses `zenoh` as its core communication middleware. Ensure you have it included in your `Cargo.toml` file:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
zenoh = { version = "1.1.0" }
```

### Setting up the Zenoh Router
Use the `router.json5` file provided in the `image-publisher` package to configure and run the Zenoh router:

```bash
zenohd -c router.json5
```

### Clone and Build the Project

Clone this repository:

```bash
git clone https://github.com/your-repo/image-subscriber.git
cd image-subscriber
```

Build the project:

```bash
cargo build --release
```

Run the project:

```bash
cargo run
