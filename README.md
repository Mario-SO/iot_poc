# 🌐 Distributed Worker-Server System in Rust

Welcome to our Distributed Worker-Server System project! This system, written in the powerful Rust programming language, showcases a network of worker nodes communicating with a central server. Each worker node, upon starting, establishes a connection with the server and participates in the network communication.

## Features 🌟

- **Rust-Powered**: Leverages Rust's excellent concurrency and safety features for network operations.
- **Server-Worker Architecture**: Implements a basic model where workers connect to and communicate with a central server.
- **Asynchronous Communication**: Uses asynchronous programming for efficient, non-blocking network communication.
- **Scalable Design**: The architecture is designed with scalability in mind, allowing for an increase in the number of worker nodes.

## Getting Started 🚀

### Prerequisites

- Ensure you have Rust and Cargo installed on your machine. Visit [Rust's official site](https://www.rust-lang.org/tools/install) for installation instructions.
- Basic understanding of Rust and network programming.

### Installation

Clone the repository to get started:

```bash
git clone https://github.com/your-username/iot_poc.git
cd iot_poc
```

## Usage 🖥️

### Running the Server

Navigate to the server project's directory:

```bash
cd server
cargo run
```

The server will start and listen for incoming connections from worker nodes.

### Running Worker Nodes

In a new terminal, navigate to the worker project's directory:

```bash
cd workers
cargo run
```

This will start a worker node that connects to the server. You can open multiple terminals and run this command to simulate multiple worker nodes.

## How It Works 🔍

- **Server**: The server listens on a specified port for incoming connections from workers. It can handle multiple worker connections simultaneously.
- **Workers**: Each worker establishes a connection with the server on startup. They then communicate with the server, sending and receiving messages.

## Contributing 🤝

Contributions to this project are welcome! Feel free to fork the repo, make your changes, and submit a pull request.

---

⌨️ with ❤️ by [mariodev](https://github.com/Mario-SO) 🚀

---
