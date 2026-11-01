# todo-api

A small todo REST API I'm building to learn Rust. It's not meant to be
anything serious, just a place to get comfortable with axum, async Rust, and
structuring a project beyond a single `main.rs`.

## What it does so far

- `GET /todos` - returns the list of todos
- `GET /todos/{id}` - returns a single todo, or 404 if it doesn't exist

Right now the todos live in a static in-memory list, so there's no persistence
yet and nothing is actually populated. That's next on my list.

## Running it

You'll need a reasonably recent Rust toolchain (this uses the 2024 edition).

```sh
cargo run
```

The server listens on `0.0.0.0:5001`. Try it with:

```sh
curl http://localhost:5001/todos
```
