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
- [ ] 5) Basic Routing & Request Handling

### Phase 2: Concurrency & Performance
- [ ] 6) Thread Pool Architecture
- [ ] 7) Memory Pools & Zero-Copy Buffers
- [ ] 8) HTTP/1.1 & Connection Pooling
- [ ] 9) Epoll/Kqueue Event Loop
- [ ] 10) Multi-threaded Event Loop

### Phase 3: Advanced Parsing & Optimisation
- [ ] 11) SIMD Parser Optimisations
- [ ] 12) Lock-Free Statistics & Metrics
- [ ] 13) Request Router & Pattern Matching
- [ ] 14) Response Caching & ETags

### Phase 4: Kernel Bypass & Advanced I/O
- [ ] 15) io_uring Integration
- [ ] 16) Sendfile & Splice Operations
- [ ] 17) SO_REUSEPORT Load Balancing
- [ ] 18) TCP_DEFER_ACCEPT & TCP_QUICKACK

### Phase 5: Security & Robustness
- [ ] 19) Rate Limiting & DDoS Protection
- [ ] 20) Request Smuggling & Parser Security
- [ ] 21) Timeouts, Backpressure & Circuit Breakers
- [ ] 22) TLS/SSL Implementation Basics

### Phase 6: Modern Protocols
- [ ] 23) HTTP/2 Frame Processing
- [ ] 24) HTTP/2 Stream Multiplexing
- [ ] 25) WebSocket Protocol Upgrade
- [ ] 26) Server-Sent Events (SSE)

### Phase 7: UDP & Alternative Protocols
- [ ] 27) UDP Server Fundamentals
- [ ] 28) Reliable UDP Implementation
- [ ] 29) Multicast & Broadcast
- [ ] 30) QUIC Concepts & 0-RTT

---

## 📍 Learning Milestones

### 1) Raw Sockets & System Calls

#### Learning Objectives
- Understand Berkeley sockets API and its evolution
- Learn the syscall boundary between userspace and kernel
- Explore file descriptors and their lifecycle
- Understand blocking vs non-blocking I/O fundamentals

#### Implementation Scope
- [ ] Create TCP socket and bind to localhost:3000
- [ ] Implement listening with appropriate backlog
- [ ] Accept incoming connections and log client addresses
- [ ] Read and echo data with proper buffer management
- [ ] Implement graceful shutdown sequence
- [ ] Add SO_REUSEADDR to prevent address conflicts
- [ ] Disable Nagle's algorithm with TCP_NODELAY
- [ ] Handle EINTR, EAGAIN, and EWOULDBLOCK errors
- [ ] Add strace instrumentation for syscall analysis
- [ ] Log syscall timings and frequencies

#### Conceptual Exploration
- [ ] Compare syscall overhead with different buffer sizes
- [ ] Test blocking behaviour with slow clients
- [ ] Trigger and handle file descriptor exhaustion
- [ ] Compare different write methods (send, write, writev)
- [ ] Observe TIME_WAIT state accumulation patterns
- [ ] Experiment with SO_LINGER settings
- [ ] Profile context switches during concurrent connections

#### Observability & Measurement
- [ ] Count syscalls per connection lifecycle
- [ ] Measure syscall latency distribution
- [ ] Track file descriptor usage and limits
- [ ] Monitor kernel buffer sizes and utilisation
- [ ] Analyse kernel vs userspace CPU time
- [ ] Record connection setup and teardown times

#### Failure Scenarios
- [ ] Handle file descriptor limit errors
- [ ] Detect and recover from connection resets
- [ ] Handle broken pipe errors gracefully
- [ ] Test partial write scenarios
- [ ] Recover from interrupted system calls
- [ ] Handle connection timeout errors

#### Exit Criteria
- [ ] Handle 100 concurrent connections without leaks
- [ ] Zero memory leaks confirmed with valgrind
- [ ] Documented syscall cost comparison table
- [ ] Clean shutdown within 100ms
- [ ] Correct error handling for all edge cases

---

### 2) TCP State Machine & Connection Handling

#### Learning Objectives
- Understand TCP states and transitions in practice
- Learn about connection establishment and teardown costs
- Explore keepalive, linger, and timeout behaviours
- Understand TCP's reliability mechanisms

#### Implementation Scope
- [ ] Define and track all TCP states per connection
- [ ] Build connection state tracking system
- [ ] Implement TCP keepalive configuration
- [ ] Add connection inactivity timeout detection
- [ ] Verify state against kernel TCP state
- [ ] Handle half-close scenarios properly
- [ ] Implement FIN_WAIT_2 timeout handling
- [ ] Create connection pool with pre-allocation
- [ ] Log all state transitions with timestamps
- [ ] Track time spent in each state
- [ ] Handle RST packet detection

#### Conceptual Exploration
- [ ] Measure three-way handshake timing
- [ ] Test SYN retransmission behaviour
- [ ] Trigger and observe keepalive probes
- [ ] Study TIME_WAIT accumulation patterns
- [ ] Experiment with half-duplex connections
- [ ] Measure memory usage per connection state
- [ ] Compare different timeout configurations

#### Observability & Measurement
- [ ] Track connection state distribution
- [ ] Measure state transition latencies
- [ ] Count transitions per second
- [ ] Monitor bytes transferred per connection
- [ ] Track TIME_WAIT socket counts
- [ ] Measure per-state memory overhead

#### Failure Scenarios
- [ ] Handle SYN flood attacks
- [ ] Detect half-open connections
- [ ] Prevent connection state leaks
- [ ] Handle RST injection attacks
- [ ] Test TIME_WAIT exhaustion
- [ ] Verify accept queue overflow handling

#### Exit Criteria
- [ ] Correctly track 1000 connection lifecycles
- [ ] No connection leaks after stress testing
- [ ] Accurate per-connection memory measurement
- [ ] Complete state transition documentation
- [ ] TIME_WAIT count stays manageable

---

### 3) Thread-per-Connection Model

#### Learning Objectives
- Understand thread creation overhead and limits
- Learn pthread basics and synchronisation
- Explore thread safety and data races
- Understand why this model doesn't scale

#### Implementation Scope
- [ ] Spawn thread for each accepted connection
- [ ] Pass connection context to threads safely
- [ ] Implement thread-safe logging system
- [ ] Add atomic connection counter
- [ ] Set meaningful thread names
- [ ] Configure appropriate stack sizes
- [ ] Implement graceful shutdown mechanism
- [ ] Add thread-local storage for statistics
- [ ] Create thread pool precursor
- [ ] Track thread lifecycle events
- [ ] Monitor system thread count
- [ ] Add thread panic recovery

#### Conceptual Exploration
- [ ] Measure thread creation overhead at scale
- [ ] Find maximum sustainable thread count
- [ ] Profile context switch frequency
- [ ] Compare synchronisation primitives
- [ ] Measure stack memory consumption
- [ ] Test false sharing scenarios
- [ ] Analyse throughput vs thread count

#### Observability & Measurement
- [ ] Track thread creation rate
- [ ] Monitor per-thread context switches
- [ ] Measure lock contention time
- [ ] Track memory usage breakdown
- [ ] Profile per-thread CPU usage
- [ ] Monitor scheduler queue length

#### Failure Scenarios
- [ ] Handle thread creation failures
- [ ] Detect and prevent thread leaks
- [ ] Identify and resolve deadlocks
- [ ] Test stack overflow conditions
- [ ] Handle thundering herd problem
- [ ] Recover from thread panics

#### Exit Criteria
- [ ] Support 1000 concurrent connections
- [ ] Clean shutdown of all threads
- [ ] Measured thread creation overhead
- [ ] Documented scalability limits
- [ ] No resource leaks detected

---

### 4) HTTP/1.0 Parser & Generator

#### Learning Objectives
- Understand HTTP message structure and parsing rules
- Learn about protocol state machines
- Explore parser security and edge cases
- Apply compiler techniques to parsing

#### Implementation Scope
- [ ] Define parser state machine states
- [ ] Parse HTTP methods (GET, POST, etc.)
- [ ] Extract request path with validation
- [ ] Validate HTTP version string
- [ ] Parse headers into key-value structure
- [ ] Handle header line continuations
- [ ] Implement case-insensitive header matching
- [ ] Parse and validate Content-Length
- [ ] Read request body correctly
- [ ] Generate valid HTTP/1.0 responses
- [ ] Add required response headers
- [ ] Format dates in RFC 822 format
- [ ] Ensure proper CRLF line endings

#### Conceptual Exploration
- [ ] Compare parsing strategies performance
- [ ] Test with various request sizes
- [ ] Attempt header injection attacks
- [ ] Profile branch prediction accuracy
- [ ] Compare parser architectures
- [ ] Test with different HTTP versions
- [ ] Measure memory allocation patterns

#### Observability & Measurement
- [ ] Track parsing time per request
- [ ] Count state machine transitions
- [ ] Measure memory allocated during parsing
- [ ] Log parser state changes
- [ ] Profile CPU cycles per byte
- [ ] Monitor cache behaviour

#### Failure Scenarios
- [ ] Reject oversized request lines
- [ ] Handle missing or malformed CRLFs
- [ ] Reject invalid headers
- [ ] Handle invalid Content-Length values
- [ ] Prevent header bomb attacks
- [ ] Reject oversized header values
- [ ] Handle incomplete requests
- [ ] Return appropriate error responses

#### Exit Criteria
- [ ] Parse 10K requests/sec single-threaded
- [ ] Survive fuzzer testing
- [ ] Reject all malformed requests properly
- [ ] Complete parsing cost analysis
- [ ] Full RFC 1945 compliance

---

### 5) Basic Routing & Request Handling

#### Learning Objectives
- Understand request dispatch and routing patterns
- Learn about URL path matching algorithms
- Explore RESTful design principles
- Implement request/response lifecycle

#### Implementation Scope
- [ ] Create route registration system
- [ ] Support exact path matching
- [ ] Implement method-specific routing
- [ ] Add path parameter extraction
- [ ] Support wildcard routes
- [ ] Parse query strings into parameters
- [ ] Implement URL decoding
- [ ] Build request context object
- [ ] Create response builder
- [ ] Add 404 handler
- [ ] Support HEAD requests properly
- [ ] Implement request logging

#### Conceptual Exploration
- [ ] Compare route lookup algorithms
- [ ] Test routing with many routes
- [ ] Verify route priority ordering
- [ ] Measure routing memory overhead
- [ ] Test regex-based routing
- [ ] Benchmark query string parsing
- [ ] Measure URL decoding impact

#### Observability & Measurement
- [ ] Track route lookup time
- [ ] Count requests per route
- [ ] Measure handler vs framework time
- [ ] Log slow request handling
- [ ] Track 404 rates and patterns
- [ ] Monitor per-request allocations

#### Failure Scenarios
- [ ] Prevent circular redirects
- [ ] Block directory traversal attempts
- [ ] Handle URL encoding errors
- [ ] Reject oversized query strings
- [ ] Detect unreachable routes
- [ ] Recover from handler crashes

#### Exit Criteria
- [ ] Route 100K requests/sec
- [ ] Efficient parameter extraction
- [ ] RFC 3986 compliant URLs
- [ ] Complete routing analysis
- [ ] Robust error handling

---

### 6) Thread Pool Architecture

#### Learning Objectives
- Understand work queue patterns and scheduling
- Learn about queue contention and false sharing
- Explore task batching and amortisation
- Apply HFT-style lock-free techniques

#### Implementation Scope
- [ ] Create fixed-size thread pool
- [ ] Implement task queue with mutex/condition
- [ ] Define task abstraction
- [ ] Add task submission interface
- [ ] Set appropriate queue capacity
- [ ] Implement task scheduling strategy
- [ ] Add graceful shutdown
- [ ] Support task cancellation
- [ ] Implement work stealing
- [ ] Add per-thread local queues
- [ ] Support batch task submission
- [ ] Monitor queue depth

#### Conceptual Exploration
- [ ] Find optimal thread count
- [ ] Compare scheduling strategies
- [ ] Test queue designs
- [ ] Measure lock contention
- [ ] Evaluate work stealing benefits
- [ ] Check for false sharing
- [ ] Analyse throughput vs latency

#### Observability & Measurement
- [ ] Track queue depth over time
- [ ] Measure task latency
- [ ] Monitor work distribution
- [ ] Track thread utilisation
- [ ] Count work stealing events
- [ ] Measure lock hold times

#### Failure Scenarios
- [ ] Handle queue overflow
- [ ] Recover from worker crashes
- [ ] Prevent priority inversion
- [ ] Handle task exceptions
- [ ] Test shutdown with pending work
- [ ] Break task submission deadlocks

#### Exit Criteria
- [ ] Process 50K tasks/sec
- [ ] Achieve fair work distribution
- [ ] Maintain bounded latency
- [ ] Complete clean shutdown
- [ ] Optimal configuration documented

---

### 7) Memory Pools & Zero-Copy Buffers

#### Learning Objectives
- Understand allocation overhead and fragmentation
- Learn about arena allocation patterns
- Explore zero-copy techniques and page flipping
- Apply GPU-style memory pooling concepts

#### Implementation Scope
- [ ] Create fixed-size buffer pool
- [ ] Implement efficient free list
- [ ] Add constant-time alloc/free
- [ ] Use mmap for backing memory
- [ ] Align buffers to cache lines
- [ ] Build ring buffer structure
- [ ] Implement scatter-gather I/O
- [ ] Add sendfile support
- [ ] Use splice for zero-copy
- [ ] Create arena allocator
- [ ] Add memory debugging features
- [ ] Track pool statistics

#### Conceptual Exploration
- [ ] Compare with malloc performance
- [ ] Test cache alignment benefits
- [ ] Measure zero-copy gains
- [ ] Test huge page impact
- [ ] Check for false sharing
- [ ] Evaluate arena patterns
- [ ] Benchmark file serving methods

#### Observability & Measurement
- [ ] Track allocation latency
- [ ] Monitor pool utilisation
- [ ] Measure fragmentation
- [ ] Count page faults
- [ ] Track cache misses
- [ ] Monitor memory growth

#### Failure Scenarios
- [ ] Handle pool exhaustion
- [ ] Detect use-after-free
- [ ] Find memory leaks
- [ ] Detect buffer overflows
- [ ] Handle allocation failures
- [ ] Eliminate false sharing

#### Exit Criteria
- [ ] 10x faster than malloc
- [ ] Zero steady-state allocations
- [ ] 50% CPU reduction for files
- [ ] Optimal pool sizing determined
- [ ] Memory safety verified

---

### 8) HTTP/1.1 & Connection Pooling

#### Learning Objectives
- Understand persistent connections and pipelining
- Learn about connection reuse benefits
- Explore head-of-line blocking issues
- Understand chunked encoding

#### Implementation Scope
- [ ] Parse Connection header properly
- [ ] Parse Keep-Alive parameters
- [ ] Implement connection timeouts
- [ ] Support HTTP/1.1 keep-alive default
- [ ] Add request pipelining
- [ ] Maintain pipeline queue
- [ ] Parse Transfer-Encoding header
- [ ] Decode chunked request bodies
- [ ] Generate chunked responses
- [ ] Build connection pool
- [ ] Implement connection eviction
- [ ] Track per-connection statistics

#### Conceptual Exploration
- [ ] Measure connection reuse savings
- [ ] Test pipeline depth effects
- [ ] Compare encoding methods
- [ ] Analyse memory per connection
- [ ] Tune keepalive timeouts
- [ ] Measure pipelining benefits
- [ ] Study head-of-line blocking

#### Observability & Measurement
- [ ] Track requests per connection
- [ ] Measure reuse rates
- [ ] Monitor pipeline depths
- [ ] Track connection lifetimes
- [ ] Count close reasons
- [ ] Measure encoding overhead

#### Failure Scenarios
- [ ] Handle pipeline desync
- [ ] Detect incomplete chunks
- [ ] Prevent connection leaks
- [ ] Handle fragmented pipelines
- [ ] Test connection limits
- [ ] Detect request smuggling

#### Exit Criteria
- [ ] Support 10K persistent connections
- [ ] Correct pipeline handling
- [ ] 90% connection reuse rate
- [ ] Optimal timeout configuration
- [ ] RFC 2616 compliance

---

### 9) Epoll/Kqueue Event Loop

#### Learning Objectives
- Understand event-driven I/O and reactor pattern
- Learn about edge vs level triggered modes
- Explore C10K problem solutions
- Understand async I/O benefits

#### Implementation Scope
- [ ] Create epoll/kqueue instance
- [ ] Add listening socket to epoll
- [ ] Implement main event loop
- [ ] Handle accept events properly
- [ ] Set sockets non-blocking
- [ ] Add client sockets with proper events
- [ ] Handle partial reads and writes
- [ ] Build connection state machine
- [ ] Implement timer wheel
- [ ] Support event modification
- [ ] Handle connection close events
- [ ] Batch accept calls

#### Conceptual Exploration
- [ ] Compare triggering modes
- [ ] Test event batch sizes
- [ ] Measure loop iteration time
- [ ] Compare with thread model
- [ ] Test multi-thread epoll
- [ ] Evaluate EPOLLONESHOT
- [ ] Measure latency at scale

#### Observability & Measurement
- [ ] Track events per second
- [ ] Measure loop iteration time
- [ ] Count spurious wakeups
- [ ] Monitor connection count
- [ ] Track timer operations
- [ ] Measure CPU efficiency

#### Failure Scenarios
- [ ] Handle epoll limits
- [ ] Prevent edge-triggered starvation
- [ ] Recover from state corruption
- [ ] Handle timer overflow
- [ ] Detect blocked event loops
- [ ] Handle FD exhaustion

#### Exit Criteria
- [ ] Handle 10K connections single-threaded
- [ ] Sub-millisecond loop iterations
- [ ] CPU scales with active connections
- [ ] Maximum event rate achieved
- [ ] No connection starvation

---

### 10) Multi-threaded Event Loop

#### Learning Objectives
- Understand event loop scaling strategies
- Learn about SO_REUSEPORT and accept() distribution
- Explore thread synchronisation in event systems
- Apply lock-free techniques to event handling

#### Implementation Scope
- [ ] Create per-CPU event loops
- [ ] Use SO_REUSEPORT for all threads
- [ ] Pin threads to CPU cores
- [ ] Implement per-thread connection tables
- [ ] Add lock-free statistics
- [ ] Use eventfd for thread communication
- [ ] Support connection migration
- [ ] Implement work stealing
- [ ] Create shared-nothing hot path
- [ ] Add cross-thread task queue
- [ ] Coordinate graceful shutdown
- [ ] Use appropriate memory barriers

#### Conceptual Exploration
- [ ] Test load distribution fairness
- [ ] Measure NUMA effects
- [ ] Compare architecture designs
- [ ] Profile cache coherence
- [ ] Test migration overhead
- [ ] Measure statistics impact
- [ ] Evaluate work stealing

#### Observability & Measurement
- [ ] Track per-thread connections
- [ ] Measure cross-thread latency
- [ ] Monitor cache misses
- [ ] Track load imbalance
- [ ] Count connection migrations
- [ ] Measure scaling efficiency

#### Failure Scenarios
- [ ] Handle uneven distribution
- [ ] Isolate thread crashes
- [ ] Prevent table corruption
- [ ] Handle mid-migration failures
- [ ] Test shutdown coordination
- [ ] Verify connection integrity

#### Exit Criteria
- [ ] Linear scaling to core count
- [ ] 100K total connections
- [ ] Less than 10% imbalance
- [ ] Optimal thread configuration
- [ ] Zero hot-path contention

---

### 11) SIMD Parser Optimisations

#### Learning Objectives
- Understand SIMD instructions and vectorisation
- Learn about data parallelism in parsing
- Apply compiler optimisation techniques
- Explore branch-free programming

#### Implementation Scope
- [ ] Detect CPU SIMD capabilities
- [ ] Implement vectorised delimiter search
- [ ] Use SIMD for byte comparisons
- [ ] Add wider vector support (AVX2)
- [ ] Vectorise whitespace operations
- [ ] Create branch-free decoders
- [ ] Validate URIs with SIMD
- [ ] Extract comparison results efficiently
- [ ] Build DFA method parser
- [ ] Add prefetch optimisations
- [ ] Maintain scalar fallback
- [ ] Ensure proper alignment

#### Conceptual Exploration
- [ ] Measure vectorisation speedup
- [ ] Test on various input sizes
- [ ] Analyse branch reduction
- [ ] Compare with auto-vectorisation
- [ ] Test alignment impact
- [ ] Measure instruction latencies
- [ ] Evaluate cache usage

#### Observability & Measurement
- [ ] Track bytes per instruction
- [ ] Measure SIMD utilisation
- [ ] Count branch mispredictions
- [ ] Monitor vectorisation ratio
- [ ] Track cache behaviour
- [ ] Measure throughput gains

#### Failure Scenarios
- [ ] Handle unaligned access
- [ ] Support non-SIMD CPUs
- [ ] Verify correctness
- [ ] Handle short inputs
- [ ] Prevent overflow bugs
- [ ] Validate against scalar

#### Exit Criteria
- [ ] 5x parsing speedup achieved
- [ ] 1M requests/sec rate
- [ ] Branch-free common path
- [ ] Complete speedup analysis
- [ ] Bit-exact with scalar

---

### 12) Lock-Free Statistics & Metrics

#### Learning Objectives
- Understand lock-free programming principles
- Learn about atomic operations and memory ordering
- Explore HFT-style metrics collection
- Apply RCU and hazard pointer patterns

#### Implementation Scope
- [ ] Create lock-free counters
- [ ] Use relaxed memory ordering
- [ ] Build per-CPU counters
- [ ] Add periodic aggregation
- [ ] Implement lock-free histogram
- [ ] Use atomic fetch-add
- [ ] Create wait-free readers
- [ ] Add lock-free averages
- [ ] Implement RCU updates
- [ ] Use hazard pointers
- [ ] Add memory barriers
- [ ] Build lock-free timers

#### Conceptual Exploration
- [ ] Measure atomic costs
- [ ] Test contention scaling
- [ ] Compare atomic operations
- [ ] Profile cache bouncing
- [ ] Test memory orderings
- [ ] Measure false sharing
- [ ] Analyse reader throughput

#### Observability & Measurement
- [ ] Track retry rates
- [ ] Measure update latency
- [ ] Monitor coherence traffic
- [ ] Track barrier overhead
- [ ] Count update success
- [ ] Verify zero reader blocking

#### Failure Scenarios
- [ ] Test ABA problems
- [ ] Verify ordering correctness
- [ ] Handle counter overflow
- [ ] Test race conditions
- [ ] Check memory reclamation
- [ ] Prevent lost updates

#### Exit Criteria
- [ ] 10M updates/sec rate
- [ ] Truly wait-free readers
- [ ] Sub-100ns update latency
- [ ] Race-free verification
- [ ] Optimal design documented

---

### 13) Request Router & Pattern Matching

#### Learning Objectives
- Understand routing algorithms and data structures
- Learn about trie, radix tree, and DFA routing
- Apply compiler techniques to pattern matching
- Explore regex engine internals

#### Implementation Scope
- [ ] Build trie-based router
- [ ] Extract route parameters
- [ ] Support wildcard patterns
- [ ] Add regex route matching
- [ ] Implement route priorities
- [ ] Cache compiled patterns
- [ ] Add middleware support
- [ ] Build route groups
- [ ] Support route prefixes
- [ ] Add route debugging
- [ ] Implement route generation
- [ ] Profile route matching

#### Conceptual Exploration
- [ ] Compare routing structures
- [ ] Measure lookup scaling
- [ ] Test cache locality
- [ ] Profile compilation time
- [ ] Evaluate regex engines
- [ ] Test parameter extraction
- [ ] Measure memory usage

#### Observability & Measurement
- [ ] Track lookup latency
- [ ] Measure memory per route
- [ ] Monitor cache misses
- [ ] Track compilation time
- [ ] Count route matches
- [ ] Profile hot paths

#### Failure Scenarios
- [ ] Prevent route explosion
- [ ] Handle regex backtracking
- [ ] Block parameter injection
- [ ] Detect shadowed routes
- [ ] Handle pattern errors
- [ ] Limit recursion depth

#### Exit Criteria
- [ ] 1M lookups/sec rate
- [ ] O(log n) complexity
- [ ] Complete performance comparison
- [ ] Safe regex handling
- [ ] Efficient parameter extraction

---

### 14) Response Caching & ETags

#### Learning Objectives
- Understand HTTP caching mechanisms
- Learn about cache invalidation strategies
- Explore ETag generation and validation
- Implement efficient cache storage

#### Implementation Scope
- [ ] Generate ETags for responses
- [ ] Parse If-None-Match headers
- [ ] Return 304 Not Modified
- [ ] Implement in-memory cache
- [ ] Add cache size limits
- [ ] Support cache expiration
- [ ] Parse Cache-Control headers
- [ ] Implement LRU eviction
- [ ] Add cache statistics
- [ ] Support cache bypassing
- [ ] Handle cache invalidation
- [ ] Add cache warming

#### Conceptual Exploration
- [ ] Compare ETag algorithms
- [ ] Test cache hit rates
- [ ] Measure memory usage
- [ ] Evaluate eviction policies
- [ ] Test invalidation patterns
- [ ] Profile lookup performance
- [ ] Analyse cache effectiveness

#### Observability & Measurement
- [ ] Track hit/miss rates
- [ ] Measure cache latency
- [ ] Monitor memory usage
- [ ] Track eviction frequency
- [ ] Count invalidations
- [ ] Profile ETag generation

#### Failure Scenarios
- [ ] Handle cache overflow
- [ ] Prevent cache poisoning
- [ ] Handle corrupt entries
- [ ] Test concurrent access
- [ ] Verify invalidation
- [ ] Handle memory pressure

#### Exit Criteria
- [ ] 90% cache hit rate
- [ ] Microsecond lookups
- [ ] Bounded memory usage
- [ ] Correct HTTP semantics
- [ ] Efficient invalidation

---

### 15) io_uring Integration

#### Learning Objectives
- Understand kernel bypass benefits and costs
- Learn about io_uring's ring buffer design
- Explore zero-syscall I/O operations
- Understand DPDK/XDP concepts

#### Implementation Scope
- [ ] Setup io_uring rings
- [ ] Implement submission queue
- [ ] Handle completion queue
- [ ] Add batched operations
- [ ] Support fixed buffers
- [ ] Implement SQ polling
- [ ] Add linked operations
- [ ] Handle async cancellation
- [ ] Support file registration
- [ ] Add timeout operations
- [ ] Implement proper cleanup
- [ ] Compare with epoll

#### Conceptual Exploration
- [ ] Measure syscall reduction
- [ ] Test polling overhead
- [ ] Profile kernel time
- [ ] Compare buffer strategies
- [ ] Test batch sizes
- [ ] Evaluate CPU usage
- [ ] Analyse latency impact

#### Observability & Measurement
- [ ] Count syscalls
- [ ] Track queue depths
- [ ] Measure CPU split
- [ ] Monitor throughput
- [ ] Track completion time
- [ ] Profile submission cost

#### Failure Scenarios
- [ ] Handle queue overflow
- [ ] Manage buffer exhaustion
- [ ] Handle reordering
- [ ] Propagate errors
- [ ] Test cancellation
- [ ] Verify cleanup

#### Exit Criteria
- [ ] 50% syscall reduction
- [ ] 2x throughput gain
- [ ] Complete performance analysis
- [ ] Proper error handling
- [ ] Documentation complete

---

### 16) Sendfile & Splice Operations

#### Learning Objectives
- Understand zero-copy file serving
- Learn about kernel buffer management
- Explore splice and tee operations
- Compare different file serving methods

#### Implementation Scope
- [ ] Implement sendfile for static files
- [ ] Add range request support
- [ ] Use splice for pipe operations
- [ ] Implement tee for duplication
- [ ] Support partial transfers
- [ ] Handle large files
- [ ] Add progress tracking
- [ ] Implement fallback paths
- [ ] Cache file descriptors
- [ ] Support compression
- [ ] Add bandwidth limiting
- [ ] Profile performance

#### Conceptual Exploration
- [ ] Compare with read/write
- [ ] Measure CPU savings
- [ ] Test with file sizes
- [ ] Profile memory usage
- [ ] Test range requests
- [ ] Evaluate caching impact
- [ ] Measure bandwidth

#### Observability & Measurement
- [ ] Track transfer rates
- [ ] Measure CPU usage
- [ ] Monitor memory bandwidth
- [ ] Count context switches
- [ ] Track cache hits
- [ ] Profile syscall overhead

#### Failure Scenarios
- [ ] Handle partial transfers
- [ ] Detect file changes
- [ ] Handle permission errors
- [ ] Test large files
- [ ] Verify cleanup
- [ ] Handle disconnections

#### Exit Criteria
- [ ] 50% CPU reduction
- [ ] Zero-copy verified
- [ ] Range requests working
- [ ] Performance documented
- [ ] Fallback paths tested

---

### 17) SO_REUSEPORT Load Balancing

#### Learning Objectives
- Understand kernel-level load balancing
- Learn about accept() distribution
- Explore CPU affinity benefits
- Compare with userspace distribution

#### Implementation Scope
- [ ] Enable SO_REUSEPORT on socket
- [ ] Create multiple acceptor threads
- [ ] Bind all to same address
- [ ] Pin threads to CPUs
- [ ] Monitor distribution fairness
- [ ] Add connection migration
- [ ] Implement fallback mechanism
- [ ] Test with different loads
- [ ] Add health checking
- [ ] Support dynamic scaling
- [ ] Profile CPU usage
- [ ] Document limitations

#### Conceptual Exploration
- [ ] Measure distribution patterns
- [ ] Test NUMA impact
- [ ] Compare with alternatives
- [ ] Profile cache effects
- [ ] Test scaling limits
- [ ] Evaluate fairness
- [ ] Measure overhead

#### Observability & Measurement
- [ ] Track per-thread accepts
- [ ] Measure imbalance
- [ ] Monitor CPU affinity
- [ ] Count migrations
- [ ] Track connection distribution
- [ ] Profile acceptance rate

#### Failure Scenarios
- [ ] Handle uneven loads
- [ ] Test thread failures
- [ ] Verify migration safety
- [ ] Handle race conditions
- [ ] Test shutdown sequence
- [ ] Check connection integrity

#### Exit Criteria
- [ ] Fair distribution achieved
- [ ] Linear scaling confirmed
- [ ] NUMA benefits documented
- [ ] Migration working
- [ ] Limitations understood

---

### 18) TCP_DEFER_ACCEPT & TCP_QUICKACK

#### Learning Objectives
- Understand TCP optimisation options
- Learn about deferred accept benefits
- Explore quick acknowledgment impact
- Study TCP option interactions

#### Implementation Scope
- [ ] Enable TCP_DEFER_ACCEPT
- [ ] Configure defer timeout
- [ ] Test with slow clients
- [ ] Enable TCP_QUICKACK
- [ ] Measure ACK reduction
- [ ] Test Nagle interaction
- [ ] Add Cork support
- [ ] Profile latency impact
- [ ] Test compatibility
- [ ] Add fallback logic
- [ ] Document trade-offs
- [ ] Create benchmarks

#### Conceptual Exploration
- [ ] Measure accept() reduction
- [ ] Test latency impact
- [ ] Profile packet counts
- [ ] Compare with defaults
- [ ] Test timeout behaviour
- [ ] Evaluate CPU savings
- [ ] Analyse network usage

#### Observability & Measurement
- [ ] Count deferred accepts
- [ ] Track ACK packets
- [ ] Measure latency change
- [ ] Monitor CPU usage
- [ ] Track packet rates
- [ ] Profile syscall reduction

#### Failure Scenarios
- [ ] Handle slow starts
- [ ] Test timeout edge cases
- [ ] Verify data integrity
- [ ] Handle option conflicts
- [ ] Test compatibility issues
- [ ] Check error handling

#### Exit Criteria
- [ ] Reduced accept overhead
- [ ] Lower packet count
- [ ] Latency impact measured
- [ ] Trade-offs documented
- [ ] Compatibility verified

---

### 19) Rate Limiting & DDoS Protection

#### Learning Objectives
- Understand rate limiting algorithms
- Learn about distributed rate limiting
- Explore probabilistic data structures
- Apply financial risk management concepts

#### Implementation Scope
- [ ] Implement token bucket algorithm
- [ ] Add sliding window counters
- [ ] Create per-IP rate limits
- [ ] Support custom keys
- [ ] Add burst allowances
- [ ] Implement gradual backoff
- [ ] Use probabilistic counting
- [ ] Add whitelist/blacklist
- [ ] Support rate limit headers
- [ ] Create admin interface
- [ ] Add distributed coordination
- [ ] Monitor effectiveness

#### Conceptual Exploration
- [ ] Compare algorithms
- [ ] Test memory efficiency
- [ ] Measure false positives
- [ ] Evaluate fairness
- [ ] Test coordination overhead
- [ ] Profile performance impact
- [ ] Analyse attack patterns

#### Observability & Measurement
- [ ] Track limit violations
- [ ] Measure memory usage
- [ ] Monitor false positives
- [ ] Track response times
- [ ] Count blocked requests
- [ ] Profile overhead

#### Failure Scenarios
- [ ] Handle coordinated attacks
- [ ] Test memory exhaustion
- [ ] Verify clock skew handling
- [ ] Test legitimate spikes
- [ ] Handle counter overflow
- [ ] Verify cleanup

#### Exit Criteria
- [ ] Accurate limiting at scale
- [ ] Sub-microsecond decisions
- [ ] Memory bounded
- [ ] Algorithm comparison complete
- [ ] Production ready

---

### 20) Request Smuggling & Parser Security

#### Learning Objectives
- Understand protocol ambiguities and security
- Learn about desync attacks
- Explore differential testing
- Apply formal verification concepts

#### Implementation Scope
- [ ] Build multiple parsers
- [ ] Add differential testing
- [ ] Implement strict mode
- [ ] Detect smuggling attempts
- [ ] Add security headers
- [ ] Validate all inputs
- [ ] Implement request limits
- [ ] Add anomaly detection
- [ ] Create security tests
- [ ] Fuzz continuously
- [ ] Document vulnerabilities
- [ ] Add monitoring

#### Conceptual Exploration
- [ ] Find parser differences
- [ ] Test ambiguous cases
- [ ] Measure security overhead
- [ ] Compare strategies
- [ ] Evaluate false positives
- [ ] Test known attacks
- [ ] Analyse patterns

#### Observability & Measurement
- [ ] Track discrepancies
- [ ] Count rejections
- [ ] Measure overhead
- [ ] Monitor coverage
- [ ] Track attack patterns
- [ ] Profile performance

#### Failure Scenarios
- [ ] Test desync attacks
- [ ] Verify injection blocks
- [ ] Handle parser bugs
- [ ] Test integer overflows
- [ ] Verify error handling
- [ ] Check fuzzer findings

#### Exit Criteria
- [ ] Zero successful attacks
- [ ] All CVEs handled
- [ ] Performance acceptable
- [ ] Fuzzer clean
- [ ] Security documented

---

### 21) Timeouts, Backpressure & Circuit Breakers

#### Learning Objectives
- Understand timeout hierarchies and cascades
- Learn about backpressure patterns
- Explore circuit breaker design
- Apply distributed systems resilience

#### Implementation Scope
- [ ] Create timeout hierarchy
- [ ] Implement cascading timeouts
- [ ] Add backpressure signals
- [ ] Build circuit breaker states
- [ ] Support half-open testing
- [ ] Add failure detection
- [ ] Implement recovery logic
- [ ] Create health checks
- [ ] Add adaptive timeouts
- [ ] Support manual controls
- [ ] Monitor state transitions
- [ ] Document patterns

#### Conceptual Exploration
- [ ] Model timeout cascades
- [ ] Test backpressure strategies
- [ ] Evaluate breaker thresholds
- [ ] Measure recovery times
- [ ] Test adaptation algorithms
- [ ] Profile overhead
- [ ] Analyse failure patterns

#### Observability & Measurement
- [ ] Track timeout rates
- [ ] Monitor backpressure
- [ ] Count breaker trips
- [ ] Measure recovery time
- [ ] Track state changes
- [ ] Profile latency

#### Failure Scenarios
- [ ] Test cascade storms
- [ ] Handle deadlocks
- [ ] Prevent flapping
- [ ] Test slow clients
- [ ] Verify recovery
- [ ] Handle overload

#### Exit Criteria
- [ ] Stable under load
- [ ] No cascading failures
- [ ] Adaptive behaviour working
- [ ] Recovery verified
- [ ] Patterns documented

---

### 22) TLS/SSL Implementation Basics

#### Learning Objectives
- Understand TLS handshake protocol
- Learn about cipher suites and negotiation
- Explore certificate validation
- Understand session resumption

#### Implementation Scope
- [ ] Implement TLS 1.2 handshake
- [ ] Support cipher negotiation
- [ ] Parse certificates
- [ ] Validate certificate chains
- [ ] Add session caching
- [ ] Support resumption
- [ ] Implement renegotiation
- [ ] Add OCSP support
- [ ] Handle alerts properly
- [ ] Support SNI
- [ ] Add performance monitoring
- [ ] Document security

#### Conceptual Exploration
- [ ] Measure handshake cost
- [ ] Test resumption benefits
- [ ] Profile crypto operations
- [ ] Compare TLS versions
- [ ] Evaluate cipher performance
- [ ] Test certificate validation
- [ ] Analyse session patterns

#### Observability & Measurement
- [ ] Track handshake time
- [ ] Count resumptions
- [ ] Measure CPU usage
- [ ] Monitor cipher selection
- [ ] Track validation time
- [ ] Profile memory usage

#### Failure Scenarios
- [ ] Test invalid certificates
- [ ] Handle downgrades
- [ ] Prevent renegotiation attacks
- [ ] Test session hijacking
- [ ] Verify alert handling
- [ ] Check crypto failures

#### Exit Criteria
- [ ] Working TLS handshake
- [ ] 1000 handshakes/sec
- [ ] Certificate validation correct
- [ ] Resumption working
- [ ] Security verified

---

### 23) HTTP/2 Frame Processing

#### Learning Objectives
- Understand binary protocol framing
- Learn about stream multiplexing concepts
- Explore header compression (HPACK)
- Understand protocol negotiation (ALPN)

#### Implementation Scope
- [ ] Parse frame headers
- [ ] Handle all frame types
- [ ] Validate frame sizes
- [ ] Implement stream states
- [ ] Build HPACK encoder
- [ ] Build HPACK decoder
- [ ] Handle dynamic table
- [ ] Support ALPN negotiation
- [ ] Add HTTP/2 upgrade
- [ ] Implement settings
- [ ] Handle connection preface
- [ ] Add frame logging

#### Conceptual Exploration
- [ ] Compare with HTTP/1.1
- [ ] Measure compression ratios
- [ ] Test multiplexing benefits
- [ ] Profile frame overhead
- [ ] Evaluate table sizes
- [ ] Test negotiation paths
- [ ] Analyse patterns

#### Observability & Measurement
- [ ] Track frame rates
- [ ] Measure compression
- [ ] Monitor table size
- [ ] Count stream counts
- [ ] Track frame types
- [ ] Profile processing

#### Failure Scenarios
- [ ] Handle invalid frames
- [ ] Test HPACK bombs
- [ ] Prevent stream exhaustion
- [ ] Handle flow control
- [ ] Test protocol errors
- [ ] Verify cleanup

#### Exit Criteria
- [ ] 100K frames/sec
- [ ] 30% compression achieved
- [ ] All frame types supported
- [ ] Error handling complete
- [ ] Interoperability verified

---

### 24) HTTP/2 Stream Multiplexing

#### Learning Objectives
- Understand stream scheduling and prioritisation
- Learn about flow control windows
- Explore head-of-line blocking solutions
- Understand fairness in multiplexing

#### Implementation Scope
- [ ] Create stream scheduler
- [ ] Implement priority trees
- [ ] Handle dependencies
- [ ] Add flow control
- [ ] Track window updates
- [ ] Support stream reset
- [ ] Handle GOAWAY frames
- [ ] Implement fair queuing
- [ ] Add stream timeouts
- [ ] Monitor stream health
- [ ] Support server push
- [ ] Profile scheduling

#### Conceptual Exploration
- [ ] Test priority schemes
- [ ] Measure flow control impact
- [ ] Compare scheduling algorithms
- [ ] Evaluate fairness metrics
- [ ] Test window tuning
- [ ] Profile multiplexing gains
- [ ] Analyse blocking patterns

#### Observability & Measurement
- [ ] Track stream latencies
- [ ] Count window updates
- [ ] Measure bandwidth sharing
- [ ] Monitor priority effects
- [ ] Track reset rates
- [ ] Profile overhead

#### Failure Scenarios
- [ ] Handle priority abuse
- [ ] Test window exhaustion
- [ ] Prevent stream starvation
- [ ] Handle memory limits
- [ ] Test rapid resets
- [ ] Verify cleanup

#### Exit Criteria
- [ ] 1000 concurrent streams
- [ ] Fair bandwidth distribution
- [ ] Priority working correctly
- [ ] No starvation detected
- [ ] Performance acceptable

---

### 25) WebSocket Protocol Upgrade

#### Learning Objectives
- Understand protocol upgrade mechanism
- Learn about frame masking and opcodes
- Explore bidirectional communication patterns
- Understand WebSocket security model

#### Implementation Scope
- [ ] Handle upgrade handshake
- [ ] Validate Sec-WebSocket headers
- [ ] Generate accept key
- [ ] Parse WebSocket frames
- [ ] Implement frame masking
- [ ] Handle control frames
- [ ] Support ping/pong
- [ ] Add close handshake
- [ ] Handle fragmentation
- [ ] Support extensions
- [ ] Add compression
- [ ] Monitor connections

#### Conceptual Exploration
- [ ] Measure upgrade overhead
- [ ] Test latency benefits
- [ ] Compare with polling
- [ ] Evaluate compression
- [ ] Test fragmentation
- [ ] Profile memory usage
- [ ] Analyse patterns

#### Observability & Measurement
- [ ] Track message rates
- [ ] Measure frame overhead
- [ ] Monitor connection duration
- [ ] Count control frames
- [ ] Track compression ratio
- [ ] Profile processing

#### Failure Scenarios
- [ ] Test frame injection
- [ ] Verify masking enforcement
- [ ] Handle connection drops
- [ ] Test memory limits
- [ ] Verify close handling
- [ ] Check extension negotiation

#### Exit Criteria
- [ ] 10K concurrent WebSockets
- [ ] 100K messages/sec
- [ ] Proper frame handling
- [ ] Security verified
- [ ] Extensions working

---

### 26) Server-Sent Events (SSE)

#### Learning Objectives
- Understand SSE protocol design
- Learn about event stream format
- Explore reconnection handling
- Compare with WebSockets and polling

#### Implementation Scope
- [ ] Set correct Content-Type
- [ ] Implement event formatting
- [ ] Add event IDs
- [ ] Support reconnection
- [ ] Handle Last-Event-ID
- [ ] Implement retry timing
- [ ] Add named events
- [ ] Support comments
- [ ] Handle connection drops
- [ ] Add client tracking
- [ ] Implement broadcasting
- [ ] Monitor connections

#### Conceptual Exploration
- [ ] Compare with WebSockets
- [ ] Test reconnection behaviour
- [ ] Measure overhead
- [ ] Evaluate buffering
- [ ] Test broadcast scaling
- [ ] Profile memory usage
- [ ] Analyse use cases

#### Observability & Measurement
- [ ] Track event rates
- [ ] Monitor connections
- [ ] Measure reconnection time
- [ ] Count dropped events
- [ ] Track bandwidth usage
- [ ] Profile CPU usage

#### Failure Scenarios
- [ ] Handle client disconnects
- [ ] Test buffer overflow
- [ ] Verify reconnection
- [ ] Handle slow clients
- [ ] Test memory limits
- [ ] Check cleanup

#### Exit Criteria
- [ ] 10K concurrent SSE connections
- [ ] Reliable reconnection
- [ ] Efficient broadcasting
- [ ] Memory bounded
- [ ] Production ready

---

### 27) UDP Server Fundamentals

#### Learning Objectives
- Understand UDP characteristics and use cases
- Learn about datagram handling
- Explore packet loss patterns
- Compare with TCP trade-offs

#### Implementation Scope
- [ ] Create UDP socket
- [ ] Bind to address
- [ ] Receive datagrams
- [ ] Send responses
- [ ] Handle ICMP errors
- [ ] Support broadcast
- [ ] Add multicast receive
- [ ] Implement echo server
- [ ] Track packet statistics
- [ ] Handle fragmentation
- [ ] Test MTU limits
- [ ] Add basic protocol

#### Conceptual Exploration
- [ ] Measure packet loss
- [ ] Test reordering
- [ ] Compare with TCP
- [ ] Evaluate buffer sizes
- [ ] Test fragmentation
- [ ] Profile overhead
- [ ] Analyse patterns

#### Observability & Measurement
- [ ] Track packet rates
- [ ] Count losses
- [ ] Measure latency
- [ ] Monitor reordering
- [ ] Track fragmentation
- [ ] Profile throughput

#### Failure Scenarios
- [ ] Handle packet loss
- [ ] Test buffer overflow
- [ ] Verify ICMP handling
- [ ] Test broadcast storms
- [ ] Handle MTU issues
- [ ] Check cleanup

#### Exit Criteria
- [ ] 100K packets/sec
- [ ] Loss detection working
- [ ] Broadcast functional
- [ ] Statistics accurate
- [ ] Trade-offs documented

---

### 28) Reliable UDP Implementation

#### Learning Objectives
- Understand custom reliability protocols
- Learn about retransmission strategies
- Explore congestion control basics
- Understand acknowledgment patterns

#### Implementation Scope
- [ ] Add sequence numbers
- [ ] Implement acknowledgments
- [ ] Add retransmission timer
- [ ] Track packet state
- [ ] Implement window control
- [ ] Add duplicate detection
- [ ] Support selective ACK
- [ ] Implement RTT estimation
- [ ] Add congestion control
- [ ] Handle reordering
- [ ] Support fragmentation
- [ ] Monitor reliability

#### Conceptual Exploration
- [ ] Test retransmission strategies
- [ ] Measure overhead
- [ ] Compare with TCP
- [ ] Evaluate window sizes
- [ ] Test congestion response
- [ ] Profile performance
- [ ] Analyse patterns

#### Observability & Measurement
- [ ] Track retransmissions
- [ ] Measure RTT accuracy
- [ ] Monitor window size
- [ ] Count duplicates
- [ ] Track throughput
- [ ] Profile overhead

#### Failure Scenarios
- [ ] Handle high loss
- [ ] Test reordering
- [ ] Verify overflow handling
- [ ] Test congestion
- [ ] Handle corruption
- [ ] Check timeouts

#### Exit Criteria
- [ ] Reliable delivery verified
- [ ] 10% loss tolerance
- [ ] Congestion control working
- [ ] Performance acceptable
- [ ] Protocol documented

---

### 29) Multicast & Broadcast

#### Learning Objectives
- Understand multicast addressing and routing
- Learn about IGMP and group management
- Explore broadcast domains and storms
- Understand reliable multicast challenges

#### Implementation Scope
- [ ] Join multicast groups
- [ ] Send multicast packets
- [ ] Handle group membership
- [ ] Set TTL values
- [ ] Enable loopback
- [ ] Support source filtering
- [ ] Add broadcast support
- [ ] Implement rate limiting
- [ ] Track receivers
- [ ] Handle failures
- [ ] Test scalability
- [ ] Document limits

#### Conceptual Exploration
- [ ] Test group scaling
- [ ] Measure overhead
- [ ] Evaluate reliability
- [ ] Test storm conditions
- [ ] Profile bandwidth
- [ ] Analyse patterns
- [ ] Compare protocols

#### Observability & Measurement
- [ ] Track packet rates
- [ ] Monitor group sizes
- [ ] Measure loss rates
- [ ] Count duplicates
- [ ] Track bandwidth
- [ ] Profile efficiency

#### Failure Scenarios
- [ ] Handle storms
- [ ] Test partitions
- [ ] Verify filtering
- [ ] Handle overflow
- [ ] Test loops
- [ ] Check cleanup

#### Exit Criteria
- [ ] Multicast to 100 receivers
- [ ] Broadcast controlled
- [ ] Storm prevention working
- [ ] Efficiency measured
- [ ] Limitations documented

---

### 30) QUIC Concepts & 0-RTT

#### Learning Objectives
- Understand QUIC's improvements over TCP+TLS
- Learn about 0-RTT connections
- Explore reliable streams over UDP
- Understand connection migration

#### Implementation Scope
- [ ] Parse QUIC packets
- [ ] Handle version negotiation
- [ ] Implement basic handshake
- [ ] Support 0-RTT data
- [ ] Add stream multiplexing
- [ ] Handle connection IDs
- [ ] Support migration
- [ ] Add congestion control
- [ ] Implement loss recovery
- [ ] Handle path validation
- [ ] Test interoperability
- [ ] Document protocol

#### Conceptual Exploration
- [ ] Measure 0-RTT benefits
- [ ] Test migration seamlessness
- [ ] Compare with HTTP/2
- [ ] Evaluate overhead
- [ ] Test loss recovery
- [ ] Profile performance
- [ ] Analyse advantages

#### Observability & Measurement
- [ ] Track handshake time
- [ ] Count 0-RTT usage
- [ ] Measure migration time
- [ ] Monitor stream counts
- [ ] Track packet overhead
- [ ] Profile throughput

#### Failure Scenarios
- [ ] Test replay attacks
- [ ] Handle migration failures
- [ ] Verify version fallback
- [ ] Test amplification
- [ ] Handle corruption
- [ ] Check cleanup

#### Exit Criteria
- [ ] Basic QUIC working
- [ ] 0-RTT functional
- [ ] Migration demonstrated
- [ ] Performance measured
- [ ] Advantages documented

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
