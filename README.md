# rusty-chat

Rusty Chat is a Rust command-line tool that enables users to chat with each other within a Local Area Network (LAN) using their terminals. It provides a simple and efficient way to establish text-based communication between devices connected to the same network.

<img width="1258" alt="Screenshot 2023-08-24 alle 22 28 21" src="https://github.com/RiccardoSegala04/rusty-chat/assets/72670063/4ec600fb-a732-4bf7-8f70-ca6fb07bd643">

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Connecting to Rusty Chat](#connecting-to-rusty-chat)
  - [Accepting Rusty Chat Connections](#listening-for-incoming-connections)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)


## Installation

1. Ensure you have Rust installed. If not, you can download and install it from the official [Rust website](https://www.rust-lang.org/tools/install).

2. Clone the Rusty Chat repository to your local machine:
   ```sh
   git clone https://github.com/your-username/rusty-chat.git
   ```
   
3. Navigate to the repository's directory:
   ```sh
   cd rusty-chat
   ```
   
4. Build the project using Cargo, the Rust package manager
  ```sh
  cargo build --release
  ```

6. The compiled binary will be located at target/release/rusty-chat. You can add this binary to your system's PATH or use it with the full path.

## Usage

The Rusty Chat tool has two main modes: **connect** and **accept**.

### Listening for incoming connections
```sh
rusty-chat accept --port PORT --name YOUR_NAME
```
- **PORT** is the port number on which the tool will listen for incoming chat connections.
- **YOUR_NAME** is your chosen name for the chat session.


### Connecting to rusty-chat
To initiate a chat connection:
```sh
rusty-chat connect --destination IP:PORT --name YOUR_NAME
```
- **IP:PORT** is the IP address and port number of the target device where the other user is accepting chat connections.
- **YOUR_NAME** is your chosen name for the chat session.

## Examples

- To connect to a chat session:
```sh
rusty-chat connect --destination 192.168.1.100:5000 --name Alice
```
- To accept chat connections:
```sh
rusty-chat accept --port 5000 --name Bob
```

## Contributing

Contributions to Rusty Chat are welcome! If you find any issues or want to add enhancements, feel free to submit a pull request. Please make sure to follow the existing coding style and conventions.

- Fork the repository.
- Create a new branch.
- Make your changes and commit them with descriptive messages.
- Push your changes to your fork.
- Open a pull request to the main repository.

## License

Rusty Chat is open-source software released under the MIT License.

