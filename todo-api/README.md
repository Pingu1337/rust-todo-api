# todo-api

This is the backend of the todo app.

## Prerequisites

- [rust/cargo](https://www.rust-lang.org/learn/get-started)
- [docker](https://www.docker.com/get-started/)


## Running the project

```bash
# cd into the correct directory
cd todo-api

# build the backend
cargo build

# Start the backend
cargo run

# run the backend with the frontend and redis

# cd to the project root and run docker compose
cd ..
docker compose up -d

# for changes to take effect 
docker compose build
# followed by
docker compose up -d
```