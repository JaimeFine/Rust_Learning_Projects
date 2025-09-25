### README.md

# MiniServer with ThreadPool in Rust

A simple multithreaded HTTP server in Rust that demonstrates building a custom thread pool and handling TCP connections concurrently.

---

## ✨ Features

* **Custom thread pool:**
    * Uses `std::sync::mpsc` for job dispatch.
    * Shares the receiver via `Arc<Mutex<...>>` for safe, concurrent access.
    * Spawns worker threads that block on the queue and execute jobs.
    * **Graceful shutdown** via a new `Message::Terminate` enum variant, ensuring workers shut down cleanly.
* **Robust HTTP handling:**
    * **Continuous listening** for incoming connections without shutting down.
    * **`GET /`** → serves `hello.html`
    * **`GET /sleep`** → waits 5 seconds, then serves `hello.html`
    * **Other paths** → serves `404.html`
    * **Handles unsupported HTTP methods** with a `405 Method Not Allowed` response.
    * **Improved error handling** and a `500 Internal Server Error` for files not found on the server.

---

## 📂 Project structure

```text
.
├── src
│   ├── lib.rs      # ThreadPool and Worker implementation
│   └── main.rs     # HTTP server using the ThreadPool
├── hello.html      # Example HTML page served at "/"
├── 404.html        # Example 404 error page
└── Cargo.toml
````

-----

## 🚀 Getting started

### Prerequisites

  * **Rust (latest stable):** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Build and run

```bash
# Clone and enter the project
git clone [https://github.com/JaimeFine/Rust_Learning_Projects/miniserver.git](https://github.com/JaimeFine/Rust_Learning_Projects/miniserver.git)
cd miniserver

# Run the server
cargo run
```

The server listens on `127.0.0.1:7878`.

-----

## 🌐 Usage

```bash
# Root path
curl [http://127.0.0.1:7878/](http://127.0.0.1:7878/)

# Simulated slow request (5s)
curl [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep)

# Unknown path → 404
curl [http://127.0.0.1:7878/unknown](http://127.0.0.1:7878/unknown)

# Unsupported method → 405
curl -X POST [http://127.0.0.1:7878/](http://127.0.0.1:7878/)
```

-----

## 🧵 ThreadPool design

  * `ThreadPool::new(size)`
      * Creates a pool with `size` workers and an `mpsc::channel` for messages.
      * The receiver is shared via `Arc<Mutex<...>>`.
  * `ThreadPool::execute(job)`
      * Boxes the closure and sends it as a `Message::NewJob` to the channel.
  * **Workers (threads)**
      * Block on `receiver.lock().unwrap().recv()` inside a loop.
      * On receiving `Message::NewJob(job)`: logs and executes the job.
      * On receiving `Message::Terminate`: prints a message and exits the loop.
  * **Drop for ThreadPool**
      * Sends a `Message::Terminate` to every worker.
      * Iterates over workers and calls `thread.join()` to ensure they have finished executing their last job and have shut down.

-----

## 🧪 Behavior notes

  * **Backpressure:** Workers block on `recv()`, so jobs queue up until a worker is free.
  * **Safety:** Sharing the receiver via `Arc<Mutex<...>>` ensures only one worker dequeues a job at a time.
  * **Shutdown:** Sending an explicit `Terminate` message to each worker ensures all threads shut down cleanly and prevents panics.
  * **Error Handling:** The server now uses `Result` to propagate errors, preventing crashes from invalid requests or missing files.

-----

## 📖 Reference

  * Rust Book, Chapter 20: Building a Multithreaded Web Server
      * [https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
