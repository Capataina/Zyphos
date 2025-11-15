# Zyphos

> A network protocol laboratory built through HTTP server implementation — learning sockets, protocols, and network programming while incorporating performance patterns from compilers, trading systems, and distributed infrastructure where they naturally apply.

---

## 🎯 Project Purpose

**Zyphos** is fundamentally about understanding network protocols and systems programming through hands-on HTTP server implementation.

While the core remains networking, we incorporate relevant techniques from high-performance domains:

- **Network Protocols** — TCP/UDP, HTTP/1.1, HTTP/2, WebSockets, QUIC, multicast
- **Parser Engineering** — applying compiler techniques to protocol parsing, SIMD optimisation
- **Concurrency Models** — thread pools, async I/O, lock-free structures from HFT systems  
- **Memory Management** — arena allocation, zero-copy buffers, GPU-inspired memory pools
- **Time & Ordering** — NTP, PTP, vector clocks, distributed consensus basics
- **Performance Engineering** — CPU pinning, NUMA awareness, kernel bypass concepts
- **Protocol Design** — state machines, schema evolution, backward compatibility

Each milestone teaches networking fundamentals while drawing performance and design patterns from production systems in finance, compilers, and distributed infrastructure.

---

## 🔬 What Makes Zyphos Different?

- **Network-first learning** — every concept taught through networking problems
- **Production patterns** — implement the same techniques used in nginx, HAProxy, and exchange gateways
- **Deliberate evolution** — build HTTP/1.0 → HTTP/1.1 → HTTP/2 to understand why protocols evolve
- **Cross-domain techniques** — apply SIMD from compilers to parsing, lock-free from HFT to connection handling
- **Measurement obsessed** — syscall counts, cache misses, packet rates, latency percentiles
- **Security through implementation** — understand vulnerabilities by creating them, then fixing them

---

## 🏗️ Core Learning Principles

- **Bottom-up networking** — raw sockets → TCP → HTTP → advanced protocols
- **Empirical understanding** — measure every syscall, every cache miss, every microsecond
- **Production relevance** — learn patterns used in real load balancers, proxies, and API gateways
- **Failure-driven learning** — trigger slowloris, SYN floods, request smuggling
- **Hardware-aware networking** — understand NICs, kernel buffers, zero-copy, DPDK
- **Protocol archaeology** — implement old versions to understand modern solutions

---

## 📚 Learning Roadmap

### Phase 1: Network Foundations
- [ ] 1) Raw Sockets & System Calls
- [ ] 2) TCP State Machine & Connection Handling
- [ ] 3) Thread-per-Connection Model
- [ ] 4) HTTP/1.0 Parser & Generator

### Phase 2: Concurrency & Performance
- [ ] 5) Thread Pool Architecture
- [ ] 6) Memory Pools & Zero-Copy Buffers
- [ ] 7) HTTP/1.1 & Connection Pooling
- [ ] 8) Epoll/Kqueue Event Loop

### Phase 3: Advanced Parsing & Optimisation
- [ ] 9) SIMD Parser Optimisations
- [ ] 10) Lock-Free Statistics & Metrics
- [ ] 11) io_uring & Kernel Bypass Concepts
- [ ] 12) Request Router & Pattern Matching

### Phase 4: Security & Robustness
- [ ] 13) Rate Limiting & DDoS Protection
- [ ] 14) Request Smuggling & Parser Security
- [ ] 15) TLS/SSL Implementation Basics
- [ ] 16) Timeouts, Backpressure & Circuit Breakers

### Phase 5: Modern Protocols
- [ ] 17) HTTP/2 Frames & Multiplexing
- [ ] 18) WebSocket Upgrade & Bidirectional Comms
- [ ] 19) UDP Server & Reliability Layers
- [ ] 20) QUIC Concepts & Multicast

---

## 📍 Learning Milestones

### 1) Raw Sockets & System Calls

#### Learning Objectives
- Understand Berkeley sockets API and its evolution
- Learn the syscall boundary between userspace and kernel
- Explore file descriptors and their lifecycle
- Understand blocking vs non-blocking I/O fundamentals

#### Implementation Scope
- [ ] Create TCP socket using raw socket(), bind(), listen(), accept()
- [ ] Build echo server reading and writing byte-by-byte
- [ ] Add proper error handling for all failure cases
- [ ] Instrument with strace to observe actual syscalls

#### Conceptual Exploration
- [ ] Measure syscall overhead with different buffer sizes
- [ ] Test impact of TCP_NODELAY on small messages
- [ ] Explore SO_REUSEADDR and port exhaustion
- [ ] Compare send() vs write() vs writev() performance

#### Observability & Measurement
- [ ] Count syscalls per connection lifecycle
- [ ] Measure syscall latency distribution
- [ ] Track file descriptor allocation patterns
- [ ] Monitor kernel buffer usage via netstat

#### Failure Scenarios
- [ ] EMFILE (too many open files)
- [ ] ECONNRESET and EPIPE handling
- [ ] Partial writes and EINTR
- [ ] TIME_WAIT state accumulation

#### Exit Criteria
- [ ] Handle 100 concurrent connections without FD leaks
- [ ] Measured: syscall cost at 1 byte vs 4KB vs 64KB buffers
- [ ] Clean shutdown handling all edge cases
- [ ] Complete syscall trace documented

---

### 2) TCP State Machine & Connection Handling

#### Learning Objectives
- Understand TCP states and transitions in practice
- Learn about connection establishment and teardown costs
- Explore keepalive, linger, and timeout behaviours
- Understand TCP's reliability mechanisms

#### Implementation Scope
- [ ] Build connection state tracker with explicit states
- [ ] Implement graceful shutdown vs abortive close
- [ ] Add connection timeout detection
- [ ] Create connection pool with reuse

#### Conceptual Exploration
- [ ] Measure three-way handshake latency
- [ ] Test keepalive probe behaviour
- [ ] Explore TCP_USER_TIMEOUT effects
- [ ] Profile TIME_WAIT accumulation patterns

#### Observability & Measurement
- [ ] Connection state distribution
- [ ] State transition latency
- [ ] Memory per connection state
- [ ] Packet capture correlation

#### Failure Scenarios
- [ ] SYN flood simulation
- [ ] Half-open connection detection
- [ ] Connection leak testing
- [ ] RST attack scenarios

#### Exit Criteria
- [ ] State machine handles all RFC transitions
- [ ] No connection leaks under stress
- [ ] Measured: memory cost per TCP connection
- [ ] Documented: when to use SO_LINGER

---

### 3) Thread-per-Connection Model

#### Learning Objectives
- Understand thread creation overhead and limits
- Learn pthread basics and synchronisation
- Explore thread safety and data races
- Understand why this model doesn't scale

#### Implementation Scope
- [ ] Spawn thread per accepted connection
- [ ] Add thread-safe logging and statistics
- [ ] Implement graceful shutdown coordination
- [ ] Build connection counter with atomics

#### Conceptual Exploration
- [ ] Measure thread creation/destruction cost
- [ ] Test maximum sustainable thread count
- [ ] Profile context switch overhead
- [ ] Compare mutex vs atomic for stats

#### Observability & Measurement
- [ ] Thread count and creation rate
- [ ] Context switches per second
- [ ] Memory usage (stack + heap)
- [ ] Lock contention profiling

#### Failure Scenarios
- [ ] Thread exhaustion limits
- [ ] Stack overflow testing
- [ ] Deadlock scenarios
- [ ] Thundering herd on accept()

#### Exit Criteria
- [ ] Handle 1000 concurrent connections
- [ ] Clean shutdown of all threads
- [ ] Measured: thread creation overhead
- [ ] Documented: why this doesn't scale

---

### 4) HTTP/1.0 Parser & Generator

#### Learning Objectives
- Understand HTTP message structure and parsing rules
- Learn about protocol state machines
- Explore parser security and edge cases
- Apply compiler techniques to parsing

#### Implementation Scope
- [ ] Parse request line and headers
- [ ] Generate valid HTTP responses
- [ ] Handle malformed requests safely
- [ ] Add basic routing logic

#### Conceptual Exploration
- [ ] Compare recursive descent vs state machine
- [ ] Measure parsing overhead per byte
- [ ] Test header injection attacks
- [ ] Profile branch prediction in parser

#### Observability & Measurement
- [ ] Parse time per request
- [ ] Memory allocations during parsing
- [ ] Branch misprediction rate
- [ ] Cache misses in parser

#### Failure Scenarios
- [ ] Malformed request handling
- [ ] Header size bombs
- [ ] Parser differential attacks
- [ ] Integer overflow in Content-Length

#### Exit Criteria
- [ ] Parse 10K requests/sec single-threaded
- [ ] Zero crashes on fuzzer input
- [ ] Measured: parsing cost breakdown
- [ ] Reject all RFC violations

---

### 5) Thread Pool Architecture

#### Learning Objectives
- Understand work queue patterns and scheduling
- Learn about queue contention and false sharing
- Explore task batching and amortisation
- Apply HFT-style lock-free techniques

#### Implementation Scope
- [ ] Build bounded thread pool with work queue
- [ ] Implement task submission and rejection
- [ ] Add work-stealing between threads
- [ ] Create graceful shutdown mechanism

#### Conceptual Exploration
- [ ] Measure optimal thread count vs cores
- [ ] Test work-stealing benefits
- [ ] Profile queue contention
- [ ] Compare LIFO vs FIFO scheduling

#### Observability & Measurement
- [ ] Queue depth histogram
- [ ] Task waiting time distribution
- [ ] Thread utilisation percentage
- [ ] Work distribution fairness

#### Failure Scenarios
- [ ] Queue overflow handling
- [ ] Worker thread crashes
- [ ] Priority inversion
- [ ] Task starvation

#### Exit Criteria
- [ ] Process 50K tasks/sec sustainably
- [ ] Fair work distribution achieved
- [ ] Measured: scheduling overhead
- [ ] Clean shutdown with pending tasks

---

### 6) Memory Pools & Zero-Copy Buffers

#### Learning Objectives
- Understand allocation overhead and fragmentation
- Learn about arena allocation patterns
- Explore zero-copy techniques and page flipping
- Apply GPU-style memory pooling concepts

#### Implementation Scope
- [ ] Build fixed-size memory pool
- [ ] Implement ring buffer for I/O
- [ ] Add scatter-gather with iovec
- [ ] Use sendfile for static content

#### Conceptual Exploration
- [ ] Measure malloc vs pool allocation
- [ ] Test cache effects of pooling
- [ ] Profile TLB misses with huge pages
- [ ] Compare copy vs zero-copy throughput

#### Observability & Measurement
- [ ] Allocation latency percentiles
- [ ] Memory fragmentation ratio
- [ ] Cache miss rates
- [ ] Memory bandwidth usage

#### Failure Scenarios
- [ ] Pool exhaustion scenarios
- [ ] Use-after-free detection
- [ ] Memory leak testing
- [ ] False sharing in pools

#### Exit Criteria
- [ ] Zero allocations in steady state
- [ ] 10x faster than malloc for fixed sizes
- [ ] Measured: CPU cycles per allocation
- [ ] Documented: when zero-copy matters

---

### 7) HTTP/1.1 & Connection Pooling

#### Learning Objectives
- Understand persistent connections and pipelining
- Learn about connection reuse benefits
- Explore head-of-line blocking issues
- Understand chunked encoding

#### Implementation Scope
- [ ] Parse Connection and Keep-Alive headers
- [ ] Implement request pipelining
- [ ] Add connection timeout management
- [ ] Support chunked transfer encoding

#### Conceptual Exploration
- [ ] Measure connection reuse savings
- [ ] Test pipelining depth effects
- [ ] Profile parsing overhead reduction
- [ ] Compare HTTP/1.0 vs 1.1 efficiency

#### Observability & Measurement
- [ ] Requests per connection
- [ ] Connection lifetime distribution
- [ ] Pipeline stall frequency
- [ ] Bandwidth utilisation

#### Failure Scenarios
- [ ] Pipeline desync attacks
- [ ] Slow request DoS
- [ ] Connection leak detection
- [ ] Timeout tuning issues

#### Exit Criteria
- [ ] 10x reduction in connection overhead
- [ ] Correct pipelining implementation
- [ ] Measured: optimal keepalive timeout
- [ ] No connection leaks under load

---

### 8) Epoll/Kqueue Event Loop

#### Learning Objectives
- Understand event-driven I/O and reactor pattern
- Learn about edge vs level triggered modes
- Explore C10K problem solutions
- Understand async I/O benefits

#### Implementation Scope
- [ ] Replace threads with event loop
- [ ] Implement epoll (Linux) / kqueue (BSD)
- [ ] Build connection state machines
- [ ] Add timer wheel for timeouts

#### Conceptual Exploration
- [ ] Compare thread vs event overhead
- [ ] Test edge vs level triggered
- [ ] Measure event batching benefits
- [ ] Profile syscall reduction

#### Observability & Measurement
- [ ] Events processed per second
- [ ] Event loop iteration time
- [ ] Connection count scalability
- [ ] CPU usage vs connections

#### Failure Scenarios
- [ ] Event starvation
- [ ] Timer wheel overflow
- [ ] FD limit exhaustion
- [ ] Spurious wakeups

#### Exit Criteria
- [ ] Handle 10K concurrent connections
- [ ] Single core saturation achieved
- [ ] Measured: events per syscall
- [ ] Sub-ms latency at 10K connections

---

### 9) SIMD Parser Optimisations

#### Learning Objectives
- Understand SIMD instructions and vectorisation
- Learn about data parallelism in parsing
- Apply compiler optimisation techniques
- Explore branch-free programming

#### Implementation Scope
- [ ] Vectorise delimiter searching
- [ ] SIMD header comparison
- [ ] Branch-free URI decoding
- [ ] Implement DFA-based routing

#### Conceptual Exploration
- [ ] Measure instructions per byte
- [ ] Test SIMD vs scalar performance
- [ ] Profile branch prediction
- [ ] Compare AVX2 vs SSE benefits

#### Observability & Measurement
- [ ] CPU cycles per request
- [ ] Branch misprediction rate
- [ ] Vector instruction usage
- [ ] Parser throughput

#### Failure Scenarios
- [ ] Alignment issues
- [ ] Short input penalties
- [ ] CPU compatibility
- [ ] Correctness vs speed

#### Exit Criteria
- [ ] 5x faster parsing vs naive
- [ ] 1M requests/sec on single core
- [ ] Measured: SIMD speedup factors
- [ ] Branch-free hot path

---

### 10) Lock-Free Statistics & Metrics

#### Learning Objectives
- Understand lock-free programming principles
- Learn about atomic operations and memory ordering
- Explore HFT-style metrics collection
- Apply RCU and hazard pointer patterns

#### Implementation Scope
- [ ] Build lock-free counters and gauges
- [ ] Implement wait-free histograms
- [ ] Add per-CPU statistics aggregation
- [ ] Create memory-barrier-free readers

#### Conceptual Exploration
- [ ] Measure atomic operation costs
- [ ] Test contention vs throughput
- [ ] Profile cache coherence traffic
- [ ] Compare CAS vs fetch-add

#### Observability & Measurement
- [ ] Metric update latency
- [ ] Cache line bouncing frequency
- [ ] Reader vs writer throughput
- [ ] Memory ordering overhead

#### Failure Scenarios
- [ ] ABA problem scenarios
- [ ] Memory ordering bugs
- [ ] Lost updates
- [ ] Integer overflow

#### Exit Criteria
- [ ] 10M updates/sec sustained
- [ ] Zero reader blocking
- [ ] Measured: CAS retry rates
- [ ] ThreadSanitizer clean

---

### 11) io_uring & Kernel Bypass Concepts

#### Learning Objectives
- Understand kernel bypass benefits and costs
- Learn about io_uring's ring buffer design
- Explore zero-syscall I/O operations
- Understand DPDK/XDP concepts

#### Implementation Scope
- [ ] Implement basic io_uring integration
- [ ] Build submission/completion queues
- [ ] Add batched I/O operations
- [ ] Compare with epoll performance

#### Conceptual Exploration
- [ ] Measure syscall elimination benefits
- [ ] Test SQ poll mode overhead
- [ ] Profile kernel crossing reduction
- [ ] Explore fixed buffers vs normal

#### Observability & Measurement
- [ ] Syscalls per operation
- [ ] Queue depth and latency
- [ ] CPU usage in kernel vs user
- [ ] I/O operation throughput

#### Failure Scenarios
- [ ] Queue overflow handling
- [ ] Buffer exhaustion
- [ ] Completion reordering
- [ ] Error propagation

#### Exit Criteria
- [ ] 50% reduction in syscalls
- [ ] 2x throughput vs epoll
- [ ] Measured: kernel bypass gains
- [ ] Documented: when to use io_uring

---

### 12) Request Router & Pattern Matching

#### Learning Objectives
- Understand routing algorithms and data structures
- Learn about trie, radix tree, and DFA routing
- Apply compiler techniques to pattern matching
- Explore regex engine internals

#### Implementation Scope
- [ ] Build trie-based router
- [ ] Add parameter extraction
- [ ] Implement wildcard patterns
- [ ] Create regex route support

#### Conceptual Exploration
- [ ] Compare routing algorithms
- [ ] Measure lookup performance
- [ ] Test cache locality effects
- [ ] Profile pattern compilation

#### Observability & Measurement
- [ ] Route lookup time
- [ ] Memory per route
- [ ] Cache misses during routing
- [ ] Compilation overhead

#### Failure Scenarios
- [ ] Route explosion DoS
- [ ] Catastrophic backtracking
- [ ] Parameter injection
- [ ] Route shadowing

#### Exit Criteria
- [ ] 1M route lookups/sec
- [ ] O(log n) worst case routing
- [ ] Measured: trie vs hash performance
- [ ] Safe regex handling

---

### 13) Rate Limiting & DDoS Protection

#### Learning Objectives
- Understand rate limiting algorithms
- Learn about distributed rate limiting
- Explore probabilistic data structures
- Apply financial risk management concepts

#### Implementation Scope
- [ ] Token bucket rate limiter
- [ ] Sliding window counters
- [ ] IP-based rate limits
- [ ] Distributed rate limiting

#### Conceptual Exploration
- [ ] Compare limiting algorithms
- [ ] Test memory vs accuracy
- [ ] Measure false positive rates
- [ ] Profile overhead impact

#### Observability & Measurement
- [ ] Limiting accuracy
- [ ] Memory per tracked entity
- [ ] Decision latency
- [ ] False positive/negative rates

#### Failure Scenarios
- [ ] Distributed coordination
- [ ] State exhaustion
- [ ] Clock skew effects
- [ ] Legitimate traffic blocking

#### Exit Criteria
- [ ] Accurate limiting at 100K IPs
- [ ] Sub-microsecond decisions
- [ ] Measured: algorithm trade-offs
- [ ] No memory exhaustion

---

### 14) Request Smuggling & Parser Security

#### Learning Objectives
- Understand protocol ambiguities and security
- Learn about desync attacks
- Explore differential testing
- Apply formal verification concepts

#### Implementation Scope
- [ ] Build differential parser testing
- [ ] Implement strict parsing mode
- [ ] Add smuggling detection
- [ ] Create security test suite

#### Conceptual Exploration
- [ ] Fuzzing for discrepancies
- [ ] Test ambiguous headers
- [ ] Profile security overhead
- [ ] Compare parser strategies

#### Observability & Measurement
- [ ] Parser discrepancy rate
- [ ] Rejection frequency
- [ ] Security check overhead
- [ ] Fuzzing coverage

#### Failure Scenarios
- [ ] CL-TE desync attacks
- [ ] Header injection
- [ ] Integer overflow
- [ ] Parser confusion

#### Exit Criteria
- [ ] Zero successful attacks
- [ ] All CVEs detected
- [ ] Measured: security vs performance
- [ ] Fuzzer finds no crashes

---

### 15) TLS/SSL Implementation Basics

#### Learning Objectives
- Understand TLS handshake protocol
- Learn about cipher suites and negotiation
- Explore certificate validation
- Understand session resumption

#### Implementation Scope
- [ ] Basic TLS 1.2 handshake
- [ ] Cipher suite negotiation
- [ ] Certificate parsing
- [ ] Session caching

#### Conceptual Exploration
- [ ] Measure handshake overhead
- [ ] Test resumption benefits
- [ ] Profile crypto operations
- [ ] Compare TLS versions

#### Observability & Measurement
- [ ] Handshake latency
- [ ] CPU per handshake
- [ ] Session cache hit rate
- [ ] Cipher suite distribution

#### Failure Scenarios
- [ ] Certificate validation bypass
- [ ] Downgrade attacks
- [ ] Renegotiation DoS
- [ ] Session hijacking

#### Exit Criteria
- [ ] Working TLS handshake
- [ ] 1000 handshakes/sec
- [ ] Measured: crypto overhead
- [ ] Secure validation chain

---

### 16) Timeouts, Backpressure & Circuit Breakers

#### Learning Objectives
- Understand timeout hierarchies and cascades
- Learn about backpressure patterns
- Explore circuit breaker design
- Apply distributed systems resilience

#### Implementation Scope
- [ ] Hierarchical timeout system
- [ ] Backpressure propagation
- [ ] Circuit breaker with states
- [ ] Adaptive timeout tuning

#### Conceptual Exploration
- [ ] Model timeout cascades
- [ ] Test backpressure strategies
- [ ] Measure circuit breaker impact
- [ ] Profile timeout overhead

#### Observability & Measurement
- [ ] Timeout trigger rates
- [ ] Backpressure engagement
- [ ] Circuit breaker state changes
- [ ] Recovery time metrics

#### Failure Scenarios
- [ ] Timeout cascade storms
- [ ] Backpressure deadlock
- [ ] Circuit breaker flapping
- [ ] Slow client attacks

#### Exit Criteria
- [ ] Stable under overload
- [ ] No cascade failures
- [ ] Measured: timeout overhead
- [ ] Adaptive recovery working

---

### 17) HTTP/2 Frames & Multiplexing

#### Learning Objectives
- Understand binary framing benefits
- Learn about stream multiplexing
- Explore HPACK compression
- Understand flow control windows

#### Implementation Scope
- [ ] Frame parser and generator
- [ ] Stream state machines
- [ ] HPACK encoder/decoder
- [ ] Priority and dependency

#### Conceptual Exploration
- [ ] Measure framing overhead
- [ ] Test multiplexing benefits
- [ ] Profile HPACK compression
- [ ] Compare with HTTP/1.1

#### Observability & Measurement
- [ ] Frames per second
- [ ] Stream concurrency
- [ ] Compression ratios
- [ ] Flow control stalls

#### Failure Scenarios
- [ ] Stream ID exhaustion
- [ ] HPACK bombs
- [ ] Priority manipulation
- [ ] Flow control deadlock

#### Exit Criteria
- [ ] 100 concurrent streams
- [ ] 30% header compression
- [ ] Measured: HTTP/2 benefits
- [ ] No head-of-line blocking

---

### 18) WebSocket Upgrade & Bidirectional Comms

#### Learning Objectives
- Understand protocol upgrade mechanism
- Learn about frame masking and opcodes
- Explore bidirectional patterns
- Understand WebSocket use cases

#### Implementation Scope
- [ ] Upgrade handshake handling
- [ ] Frame encoder/decoder
- [ ] Ping/pong heartbeats
- [ ] Message fragmentation

#### Conceptual Exploration
- [ ] Measure upgrade overhead
- [ ] Test latency vs HTTP
- [ ] Profile framing costs
- [ ] Compare with SSE

#### Observability & Measurement
- [ ] Messages per second
- [ ] Frame overhead
- [ ] Connection longevity
- [ ] Memory per WebSocket

#### Failure Scenarios
- [ ] Frame injection
- [ ] Masking bypass
- [ ] Connection hijacking
- [ ] Memory exhaustion

#### Exit Criteria
- [ ] 10K concurrent WebSockets
- [ ] 100K messages/sec
- [ ] Measured: WebSocket overhead
- [ ] Secure frame handling

---

### 19) UDP Server & Reliability Layers

#### Learning Objectives
- Understand UDP characteristics and use cases
- Learn about custom reliability protocols
- Explore congestion control basics
- Understand multicast patterns

#### Implementation Scope
- [ ] UDP echo server
- [ ] Sequence numbers and ACKs
- [ ] Simple retransmission
- [ ] Multicast support

#### Conceptual Exploration
- [ ] Measure packet loss patterns
- [ ] Test reliability overhead
- [ ] Profile UDP vs TCP
- [ ] Explore broadcast storms

#### Observability & Measurement
- [ ] Packet loss rates
- [ ] Retransmission frequency
- [ ] RTT estimation accuracy
- [ ] Throughput vs reliability

#### Failure Scenarios
- [ ] Packet loss storms
- [ ] Amplification attacks
- [ ] Congestion collapse
- [ ] NAT traversal

#### Exit Criteria
- [ ] Reliable delivery over loss
- [ ] Basic congestion control
- [ ] Measured: UDP vs TCP trade-offs
- [ ] Multicast working

---

### 20) QUIC Concepts & Multicast

#### Learning Objectives
- Understand QUIC's improvements over TCP+TLS
- Learn about 0-RTT connections
- Explore reliable multicast protocols
- Understand connection migration

#### Implementation Scope
- [ ] QUIC handshake basics
- [ ] Stream multiplexing over UDP
- [ ] Connection migration handling
- [ ] PGM/multicast implementation

#### Conceptual Exploration
- [ ] Measure 0-RTT benefits
- [ ] Test migration seamlessness
- [ ] Profile QUIC overhead
- [ ] Compare multicast efficiency

#### Observability & Measurement
- [ ] Handshake latency savings
- [ ] Migration success rate
- [ ] Packet overhead
- [ ] Multicast fanout

#### Failure Scenarios
- [ ] 0-RTT replay attacks
- [ ] Migration hijacking
- [ ] Multicast storms
- [ ] Version negotiation

#### Exit Criteria
- [ ] Basic QUIC working
- [ ] 50% handshake reduction
- [ ] Measured: QUIC vs HTTP/2
- [ ] Multicast to 100 clients

---

## 🎓 Learning Outcomes

1. **Network Programming Fundamentals** — from sockets to modern protocols
2. **Performance Engineering** — where latency comes from and how to eliminate it
3. **Concurrency Models** — threads vs events vs coroutines, and when each wins
4. **Parser Engineering** — applying compiler techniques to protocol parsing
5. **Memory Management** — allocation strategies, zero-copy techniques, cache effects
6. **Security Engineering** — how protocols fail and how to defend them
7. **Production Patterns** — the techniques used in nginx, HAProxy, and exchange gateways
8. **Hardware Awareness** — CPU caches, NUMA, kernel bypass, SIMD
9. **Distributed Systems Basics** — consensus, time, ordering, consistency
10. **Protocol Evolution** — why HTTP/2 exists, what QUIC solves, where protocols are heading
