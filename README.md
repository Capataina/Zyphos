# MultithreadedHTTP

## Project Description

**MultithreadedHTTP** is a solo Rust-based project designed to deepen my understanding of low-level internet protocols, with a particular focus on the HTTP/1.x specification. It implements a multithreaded HTTP server from scratch to expose how sockets, request parsing, routing, and thread-pool scheduling actually work. The codebase doubles as a living set of notes and a portfolio artifact showcasing systems-level craftsmanship.

---

## Features & Roadmap

- [ ] **Protocol-Level: HTTP Specification Features**
  - [ ] Parse HTTP **headers** (content negotiation, cookies, etc.)
  - [ ] Return proper **HTTP status codes** (`200 OK`, `404`, `500`, …) (in progress)
  - [ ] Support **HTTP/1.0** and **HTTP/1.1** version parsing
  - [ ] Emit **Content-Length** header
  - [ ] Honour **Connection: keep-alive / close** semantics
  - [ ] Use RFC-compliant **CRLF (`\r\n`)** line endings

- [ ] **Request Handling: Improve Input Robustness**
  - [ ] Gracefully handle **malformed requests** (no crashes)
  - [ ] Parse **query parameters** (`/search?q=term`)
  - [ ] Decode **form-encoded POST bodies**
  - [ ] Accept **JSON** POST bodies (API-style)
  - [ ] Handle **GET / POST / PUT / DELETE** methods

- [ ] **Routing System Enhancements**
  - [ ] Dynamic route parameters: `/user/:id`
  - [ ] Route **pattern matching** for scalable dispatch
  - [ ] Route grouping prefixes (e.g., `/api/*`, `/admin/*`)
  - [ ] Custom **404** page with helpful debug output

- [ ] **Concurrency & Performance**
  - [ ] Spawn a **thread per connection**
  - [ ] Build a bounded **thread pool** to protect the OS
  - [ ] Enforce **request timeouts**
  - [ ] Per-connection **logging** (timings, status codes)
  - [ ] Benchmark **latency / throughput** for optimisation

- [ ] **Server Usability Features**
  - [ ] Config via **`config.toml` / env vars** (port, threads…)
  - [ ] Strongly-typed **`Request` / `Response`** structs
  - [ ] Modular layout (`controllers`, `utils`, `router`)
  - [ ] **Response builders / macros** to avoid header boilerplate

- [ ] **Advanced: Real-World-Like Behaviour**
  - [ ] Serve over **HTTPS** with self-signed TLS
  - [ ] Serve **static files** (`/index.html`, images, JS)
  - [ ] Inject **CORS** headers for browser clients
  - [ ] Route-based **file downloads** (`/files/:name`)
  - [ ] Basic **cookie parsing & setting** for stateful flows

- [ ] **Bonus Extras**
  - [ ] **Hot reload** on source change (watch mode)
  - [ ] Auto-generate `routes.json` / docs map
  - [ ] Comprehensive **test suite** (happy & malformed paths)
