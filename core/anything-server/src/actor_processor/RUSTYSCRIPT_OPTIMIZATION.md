# RustyScript Optimization Guide for Actor Model

## Problem Summary

The server crashes when using RustyScript (JavaScript execution) within the Tokio-based actor model. The crash occurs due to:

1. **Runtime Conflicts**: RustyScript uses V8 engine internally which creates its own runtime, conflicting with Tokio
2. **Thread Pool Exhaustion**: Each JS execution spawns a blocking task, potentially exhausting the thread pool
3. **Memory Issues**: Potential memory leaks under high concurrent load
4. **Panic Propagation**: Unhandled panics in RustyScript causing server crashes

## Solutions Implemented

### 1. Enhanced Error Handling

Added comprehensive error handling with panic catching:

- Wrap RustyScript execution in `std::panic::catch_unwind`
- Proper error propagation and logging
- Graceful degradation instead of crashes

### 2. Retry Logic

Implemented retry mechanism for transient failures:

- Up to 3 attempts (initial + 2 retries)
- Small delay between retries
- Different error handling for different failure types

### 3. Better Timeout Management

Adjusted timeout strategy:

- Inner timeout (RustyScript): 25s for JS, 12s for filters
- Outer timeout (Tokio): 30s for JS, 15s for filters
- Prevents timeout race conditions

### 4. Resource Management

#### Configure Tokio Runtime

Add these environment variables to your server configuration:

```bash
# Increase blocking thread pool size (default: 512)
export TOKIO_BLOCKING_THREADS=1024

# Configure task actor pool size
export TASK_ACTOR_POOL_SIZE=100

# Configure workflow actor pool size
export WORKFLOW_ACTOR_POOL_SIZE=20
```

#### Create Custom Runtime (if needed)

```rust
use tokio::runtime::Builder;

let runtime = Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    .max_blocking_threads(1024)  // Increase from default 512
    .thread_name("anything-worker")
    .enable_all()
    .build()
    .unwrap();
```

### 5. Monitoring and Debugging

Add these log lines to help debug issues:

```rust
// Log blocking thread pool usage
if let Ok(handle) = tokio::runtime::Handle::try_current() {
    let metrics = handle.metrics();
    info!(
        "Blocking threads: active={}, queued={}",
        metrics.num_blocking_threads(),
        metrics.blocking_queue_depth()
    );
}
```

## Additional Recommendations

### 1. Consider Alternative JS Runtimes

If crashes persist, consider alternatives:

- **quickjs-rs**: Lightweight, no separate runtime
- **boa**: Pure Rust JS engine
- **deno_core**: More integrated with Tokio

### 2. Isolate JS Execution

Create a dedicated runtime for JS execution:

```rust
// Create a separate runtime for JS tasks
lazy_static! {
    static ref JS_RUNTIME: tokio::runtime::Runtime = {
        Builder::new_multi_thread()
            .worker_threads(4)  // Dedicated threads for JS
            .thread_name("js-worker")
            .enable_all()
            .build()
            .unwrap()
    };
}

// Execute JS in dedicated runtime
let result = JS_RUNTIME.spawn_blocking(|| {
    // RustyScript execution
}).await?;
```

### 3. Resource Pooling

Consider pooling RustyScript runtimes:

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

// Limit concurrent JS executions
static JS_SEMAPHORE: Lazy<Arc<Semaphore>> = Lazy::new(|| {
    Arc::new(Semaphore::new(50))  // Max 50 concurrent JS executions
});

// In execution function
let _permit = JS_SEMAPHORE.acquire().await?;
// Execute JS code
```

### 4. Circuit Breaker Pattern

Implement circuit breaker for JS execution:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static FAILURE_COUNT: AtomicU32 = AtomicU32::new(0);
const FAILURE_THRESHOLD: u32 = 10;

// Check circuit breaker before execution
if FAILURE_COUNT.load(Ordering::Relaxed) > FAILURE_THRESHOLD {
    return Err("Circuit breaker open: too many JS failures".into());
}

// On failure
FAILURE_COUNT.fetch_add(1, Ordering::Relaxed);

// On success
FAILURE_COUNT.store(0, Ordering::Relaxed);
```

## Monitoring Metrics

Add these metrics to track JS execution health:

1. **Execution Time**: Track p50, p95, p99 latencies
2. **Error Rate**: Track failure percentage
3. **Thread Pool Usage**: Monitor blocking thread utilization
4. **Memory Usage**: Track memory growth over time
5. **Panic Count**: Count and log all panics

## Testing

### Load Testing

```bash
# Simulate high concurrent JS load
for i in {1..1000}; do
    curl -X POST http://localhost:8080/execute \
        -H "Content-Type: application/json" \
        -d '{"type": "javascript", "code": "return 1+1"}' &
done
```

### Stress Testing

Use tools like:

- **wrk**: HTTP benchmarking tool
- **vegeta**: HTTP load testing tool
- **artillery**: Load testing toolkit

## Emergency Procedures

If crashes persist:

1. **Immediate**: Reduce `TASK_ACTOR_POOL_SIZE` to limit concurrency
2. **Short-term**: Disable JS execution features
3. **Long-term**: Consider alternative JS runtime implementation

## References

- [Tokio Blocking Tasks](https://tokio.rs/tokio/tutorial/bridging)
- [RustyScript Documentation](https://github.com/rscarson/rustyscript)
- [V8 Memory Management](https://v8.dev/blog/trash-talk)
- [Actor Model Best Practices](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
