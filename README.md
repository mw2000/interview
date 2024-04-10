# Rust Book API

This is a Rust project that fetches book data from the Open Library API. We also have a main.py file, which is where this rust book api was translated from.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from [the official website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/rust_book_api.git
    ```

2. Navigate to the project directory:
    ```bash
    cd rust_book_api
    ```

3. Build the project:
    ```bash
    cargo build
    ```

### Usage

To run the project, use the following command:

```bash
cargo run <isbn>
```
Alternatively, after building, you can also use 

```bash
./run.sh <isbn>
```