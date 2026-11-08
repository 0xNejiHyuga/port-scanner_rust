# 🦀 Port Scanner Rust

A fast and simple TCP port scanner written in Rust. Features a stylish ASCII banner and colored output for open ports.

## ✨ Features
- Scans a full range of TCP ports (`0-65535`).
- Colorful terminal output (green for open ports).
- Graceful error handling for invalid arguments.
- Custom ASCII-art welcome banner.

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Cargo included)

### Installation & Build
1. Clone the repository

   ```bash git clone https://github.com/0xNejiHyuga/port-scanner_rust.git```
   ```bash
   cd port-scanner_rust
   ```
2. Build the release version
  ```bash
     cargo build --release
  ```
3.Usage
  ```bash
  ./target/release/port-scanner_rust <IP> <start_port> <end_port>
  ```
Example:
  ```bash
  ./target/release/port-scanner_rust 192.168.1.1 1 1000
  ```
View/Screenshots:



<img width="531" height="763" alt="image" src="https://github.com/user-attachments/assets/e3e2c3d7-d56c-4f53-8472-0f0988818e19" />
