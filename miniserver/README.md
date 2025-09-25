### README.md

# MiniServer with ThreadPool in Rust

A simple, minimal HTTP server that handles incoming requests concurrently using a custom thread pool. It supports basic routing, delays for simulated workloads, and graceful shutdown of worker threads.

---

## ✨ Features

* **Custom thread pool:**
    * Uses `std::sync::mpsc` for job dispatch.
    * Shares the receiver via `Arc<Mutex<...>>` for safe, concurrent access.
    * Spawns worker threads that block on the queue and execute jobs.
    * Graceful shutdown via a new `Message::Terminate` enum variant, ensuring workers shut down cleanly.
* **Robust HTTP handling:**
    * Continuous listening for incoming connections without shutting down.
    * Handles multiple TCP connections concurrently 
    * `/sleep` → waits 5 seconds, then serves `hello.html`
    * Other paths → serves `404.html`
    * Handles unsupported HTTP methods with a `405 Method Not Allowed` response.
    * Improved error handling and a `500 Internal Server Error` for files not found on the server.  

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
```

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

1. **Initialization**  
   - Enforces `size > 0`  
   - Spawns `size` workers, each listening on a shared `Receiver<Message>`  
2. **Job Scheduling**  
   - `execute` accepts any `FnOnce() + Send + 'static`  
   - Wraps it in a `Message::NewJob` and sends it over the channel  
3. **Worker Loop**  
   - Locks the receiver, waits for a message  
   - On `NewJob`, prints a log and runs the closure  
   - On `Terminate`, breaks the loop and ends the thread  
4. **Graceful Shutdown**  
   - `Drop` impl sends one `Terminate` per worker  
   - Joins each worker thread before exiting 

-----

## 🦀 Differences from Rust Book Chapter 20

- The original book reads the first 512 bytes of the request using a fixed-size buffer and matches request prefixes for routing; this implementation uses `BufReader::read_line` to parse only the request line for simplicity and clarity.  

- Rust Book’s example treats any non-`/` or `/sleep` request (including non-GET methods) as a 404; this version explicitly returns `405 Method Not Allowed` for non-GET methods and `500 Internal Server Error` when requested files are missing, improving compliance with HTTP semantics.  

- Routing logic is expanded to validate request lines and provide distinct error responses rather than panicking on invalid input, adding robustness over the original tutorial’s assumptions.  

- The thread pool implementation and graceful shutdown logic closely follow the book’s design, but use Rust’s named format parameters (e.g., `{id}`) for log messages and centralize error handling in `handle_connection` for cleaner code organization.  

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


<!--
## Customization & Next Steps

- Adjust thread pool size in `main.rs` based on expected concurrency  
- Extend routing logic to serve additional files or dynamic content  
- Implement logging middleware or timeouts on connections  
- Add support for POST requests and more HTTP headers  

---

## License

This project is licensed under the MIT License.  
Feel free to copy, modify, and redistribute.  

---

References

Final Project: Building a Multithreaded Web Server - The Rust Programming Language (https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
-->