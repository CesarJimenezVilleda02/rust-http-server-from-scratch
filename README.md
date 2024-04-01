# Http Rust Server

A lightweight, efficient, and robust HTTP server implemented in Rust from scratch. This
project leverages the power of Rust's safety features, including lifetimes, to ensure
memory safety without sacrificing performance. Designed to handle TCP connections, parse
incoming data into structured data, and route requests with a custom routing
implementation, this server is perfect for developers looking for a lean yet powerful HTTP
server solution. It also includes error handling and the capability to serve static files,
making it a comprehensive package for a variety of web applications.

## Features

-   **TCP Connection Handling**: Listens for incoming TCP connections, providing a
    foundation for HTTP communications.
-   **Request Parsing**: Efficiently parses incoming data into Rust structs for easy
    handling and response generation.
-   **Custom Routing Implementation**: A flexible routing mechanism to direct requests to
    appropriate handlers based on request URIs and methods.
-   **Error Handling**: Robust error handling to manage unexpected issues gracefully,
    ensuring server stability.
-   **Static File Serving**: Ability to serve static files, enabling the server to deliver
    HTML, CSS, JavaScript, and media files directly.

## Getting Started

### Prerequisites

-   Rust Programming Language: Ensure you have the latest version of Rust and Cargo
    installed on your system. Visit
    [the Rust installation page](https://www.rust-lang.org/tools/install) for
    instructions.

### Installation

1. **Clone the Repository**

    ```sh
    git clone https://github.com/yourusername/http-rust-server.git
    cd http-rust-server
    Build the Project
    ```

Compile the server using Cargo.

cargo build --release Run the Server

Start the server with Cargo.

cargo run Alternatively, run the compiled binary directly from the target/release
directory.

Configuration Configure your server settings such as port and static file directory by
editing the Config.toml file (if applicable). Usage After starting the server, it will
listen for incoming HTTP requests. You can interact with the server using any HTTP client,
such as curl or Postman, or by accessing it through a web browser.

Example request:

curl http://localhost:8080 Replace localhost:8080 with your server's address and port.
