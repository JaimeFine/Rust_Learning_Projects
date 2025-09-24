# MiniServer with ThreadPool in Rust

A simple multithreaded HTTP server in Rust that demonstrates building a custom thread pool and handling TCP connections concurrently.

---

## ✨ Features

* **Custom thread pool:**
    * Uses `std::sync::mpsc` for job dispatch.
    * Shares the receiver via `Arc<Mutex<...>>` for safe, concurrent access.
    * Spawns worker threads that block on the queue and execute jobs.
* **Graceful shutdown:**
    * Dropping the `ThreadPool` closes the sender.
    * Workers detect disconnection, exit their loops, and `join()`.
* **Basic HTTP handling:**
    * `GET /` → serves `hello.html`
    * `GET /sleep` → waits 5 seconds, then serves `hello.html`
    * Other paths → serves `404.html`

---

## 📂 Project structure

```text
.
├── src
│   ├── lib.rs      # ThreadPool and Worker implementation
│   └── main.rs     # HTTP server using the ThreadPool
├── hello.html      # Example HTML page served at "/"
├── 404.html        # Example 404 error page
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
```

-----

## 🧵 ThreadPool design

  * `ThreadPool::new(size)`

      * Creates a pool with `size` workers, an `mpsc::channel` for jobs, and shares the receiver via `Arc<Mutex<Receiver<Job>>>`.

  * `ThreadPool::execute(job)`

      * Boxes the closure and sends it through the channel to be picked up by workers.

  * **Workers (threads)**

      * Block on `receiver.recv()` inside a loop.
      * On `Ok(job)`: log and execute the job.
      * On `Err(_)`: the sender is closed → exit the loop and shut down.

  * **Drop for ThreadPool**

      * `sender.take()` and drop it → closes the channel.
      * Iterate workers, `take()` their `JoinHandle`, and `join()` to ensure graceful shutdown.

-----

## 🧪 Behavior notes

  * **Backpressure:** Workers block on `recv()`, so jobs queue up until a worker is free.
  * **Safety:** Sharing the receiver via `Arc<Mutex<...>>` ensures only one worker dequeues a job at a time.
  * **Shutdown:** Closing the sender signals all workers to stop; joining threads prevents orphaned threads.

-----

## 📖 Reference

  * Rust Book, Chapter 20: Building a Multithreaded Web Server
      * [https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
