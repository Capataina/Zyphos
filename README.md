# Zyphos

## Project Description

**Zyphos** is a solo Rust-based project designed to deepen my understanding of low-level internet protocols, with a particular focus on the HTTP/1.x specification. It implements a multithreaded HTTP server from scratch to expose how sockets, request parsing, routing, and thread-pool scheduling actually work. The codebase doubles as a living set of notes and a portfolio artifact showcasing systems-level craftsmanship.

---

## Technologies & Dependencies

### **🦀 Core Technologies**
- **Rust 2024 Edition** - Systems programming language for memory safety and performance
- **Standard Library TCP** - Native socket programming using `std::net::TcpListener` and `std::net::TcpStream`
- **Standard Library Threading** - Rust's built-in threading primitives for concurrency

### **📦 External Dependencies**
- **[chrono 0.4](https://docs.rs/chrono/)** - Date and time library for timestamp formatting in responses

### **🏗️ Architecture & Design Patterns**
- **Modular Design** - Separated concerns across modules: routing, response handling, request parsing
- **Request-Response Pipeline** - Clear data flow from raw TCP bytes to formatted HTTP responses
- **Functional Routing** - Simple function-based routing without complex frameworks
- **Type Safety** - Leveraging Rust's type system for protocol correctness

### **🔧 Development Tools**
- **Cargo** - Rust's package manager and build system
- **Git** - Version control with `.gitignore` for build artifacts

---

## Features & Roadmap

### **🔧 Core Infrastructure & Foundations**
- [ ] **TCP Server Implementation** - Bind to address, listen for connections, accept incoming streams *(in progress)*
- [ ] **Raw HTTP parsing** - Split request line, headers, and body from byte stream *(in progress)*
- [x] **Structured Request/Response types** - Strongly-typed `Request` & `Response` structs *(in progress)*
- [ ] **Basic error handling** - Graceful failures without server crashes *(in progress)*
- [ ] **Command-line argument parsing** - Configure host, port, log level via CLI
- [ ] **Logging infrastructure** - Structured logging with timestamps and levels

### **📡 HTTP Protocol Fundamentals**
- [ ] **HTTP version validation** - Support HTTP/1.0, HTTP/1.1, reject unsupported versions
- [ ] **Request line parsing** - Extract method, path, query string, and HTTP version *(in progress)*
- [ ] **HTTP header parsing** - Parse key-value pairs, handle multi-line headers *(in progress)*
- [x] **Return proper HTTP status codes** - `200 OK`, `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`, etc. *(in progress)*
- [x] **Emit essential headers** - `Content-Length`, `Content-Type`, `Date`, `Server` *(in progress)*
- [x] **RFC-compliant formatting** - Use proper CRLF (`\r\n`) line endings *(in progress)*
- [ ] **Connection semantics** - Honor `Connection: keep-alive` vs `Connection: close`
- [ ] **HTTP method validation** - Support GET, POST, PUT, DELETE, reject invalid methods *(in progress)*

### **🔍 Request Processing & Input Validation**
- [ ] **Malformed request handling** - Gracefully handle broken HTTP without crashes *(in progress)*
- [ ] **Request size limits** - Prevent memory exhaustion from huge requests/headers
- [ ] **Query parameter parsing** - Extract and decode `/search?q=term&filter=type`
- [ ] **URL decoding** - Handle percent-encoded characters (`%20` → space)
- [ ] **Form-encoded POST parsing** - Decode `application/x-www-form-urlencoded`
- [ ] **JSON request body parsing** - Accept and validate `application/json` payloads
- [ ] **Request timeout handling** - Don't wait forever for slow clients
- [ ] **Header size validation** - Prevent header bomb attacks

### **🛣️ Routing & Response Systems**
- [ ] **Dynamic route parameters** - Handle `/user/:id`, `/posts/:slug/comments/:comment_id`
- [ ] **Route pattern matching** - Scalable dispatch system with wildcards *(in progress)*
- [ ] **Route grouping & prefixes** - Organize routes like `/api/*`, `/admin/*`
- [ ] **Custom error pages** - Rich 404/500 pages with debug info *(in progress)*
- [x] **Response builders** - Fluent API for constructing responses *(in progress)*
- [ ] **Content negotiation** - Handle `Accept` headers for different response formats
- [ ] **Redirect responses** - Support 301/302 redirects with proper `Location` headers

### **🔒 Security & Attack Prevention**
- [ ] **Input sanitization** - Prevent injection attacks in headers and body
- [ ] **Request rate limiting** - Basic DoS protection per IP address
- [ ] **Path traversal protection** - Prevent `../../../etc/passwd` attacks
- [ ] **Header injection prevention** - Validate header values for CRLF injection
- [ ] **Request method restrictions** - Block dangerous methods like TRACE
- [ ] **Host header validation** - Prevent host header injection attacks
- [ ] **Basic authentication parsing** - Handle `Authorization: Basic` headers
- [ ] **CSRF token validation** - Protect against cross-site request forgery

### **⚡ Concurrency & Performance**
- [x] **TCP connection handling** - Accept multiple simultaneous connections
- [x] **Thread-per-connection model** - Spawn worker thread for each client
- [ ] **Bounded thread pool** - Limit threads to protect OS resources
- [ ] **Connection pooling** - Reuse threads efficiently
- [ ] **Request timeout enforcement** - Kill slow/hanging requests
- [ ] **Performance logging** - Track response times, throughput metrics
- [ ] **Memory usage monitoring** - Track allocation patterns
- [ ] **Benchmarking tools** - Measure latency/throughput under load

### **🌐 Real-World Protocol Features**
- [ ] **Static file serving** - Serve HTML, CSS, JS, images with proper MIME types
- [ ] **HTTPS/TLS support** - SSL/TLS termination with self-signed certificates
- [ ] **WebSocket upgrade basics** - Handle protocol upgrade requests
- [ ] **CORS headers** - Cross-origin resource sharing for browser clients
- [ ] **Cookie parsing & setting** - Handle `Cookie` and `Set-Cookie` headers
- [ ] **Session management** - Basic stateful user sessions
- [ ] **File upload handling** - Process `multipart/form-data` requests
- [ ] **Range requests** - Support partial content (`Range: bytes=0-1023`)
- [ ] **HTTP caching** - Implement ETag, Last-Modified, Cache-Control
- [ ] **Response compression** - gzip/deflate encoding

### **🔬 Network Protocol Deep Dive**
- [ ] **TCP socket options** - Understand SO_REUSEADDR, TCP_NODELAY, keepalive
- [ ] **Network byte order** - Handle endianness in binary protocols
- [ ] **IPv4 vs IPv6** - Support both protocol versions
- [ ] **Proxy protocol support** - Handle X-Forwarded-For headers
- [ ] **DNS resolution** - Resolve hostnames to IP addresses
- [ ] **Load balancer awareness** - Health checks, graceful shutdown

### **🛠️ Development & Operations**
- [ ] **Configuration system** - TOML/YAML config files and environment variables
- [ ] **Hot reload** - Restart server on source code changes
- [ ] **Graceful shutdown** - Handle SIGTERM/SIGINT, finish current requests
- [ ] **Health check endpoints** - `/health`, `/metrics` for monitoring
- [ ] **Request/response debugging** - Verbose logging mode for development
- [ ] **Integration testing** - Test with real HTTP clients (`curl`, browsers)
- [ ] **Load testing** - Stress test with tools like `wrk` or `ab`
- [ ] **Docker containerization** - Package server in container

### **🛡️ Cybersecurity Learning**
- [ ] **Security headers** - Implement HSTS, CSP, X-Frame-Options
- [ ] **Input fuzzing** - Generate malformed requests to find crashes
- [ ] **TLS certificate validation** - Proper certificate chain verification
- [ ] **Timing attack awareness** - Consistent response times for auth
- [ ] **Network monitoring** - Detect unusual traffic patterns
- [ ] **Vulnerability scanning** - Test against common attack vectors

### **🎯 Educational Bonus Features**
- [ ] **Protocol documentation** - Auto-generate API docs from routes
- [ ] **Traffic analysis tools** - Parse and analyze HTTP logs
- [ ] **Custom middleware system** - Pluggable request/response processing
- [ ] **Database integration** - Connect to SQL databases securely
- [ ] **API versioning** - Handle `/v1/` vs `/v2/` endpoints
- [ ] **Comprehensive test suite** - Unit, integration, and fuzz testing
