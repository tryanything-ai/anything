# Actor System RustyScript Optimization

## Overview

This document describes the optimizations made to the task execution system to work better with the actor processor, particularly for JavaScript and filter tasks that use RustyScript.

## Key Changes Made

### 1. Enhanced Task Execution (`execute_task.rs`)

#### Timeout Management

- **Plugin-specific timeouts**: Different plugins now get appropriate timeouts
  - JavaScript: 60 seconds (complex operations need time)
  - Filter: 30 seconds (boolean conditions)
  - HTTP: 45 seconds (network operations)
  - Others: 15 seconds (default)
- **Phased execution**: Context bundling and plugin execution have separate timeouts
- **Better error reporting**: Timeout errors include duration and type information

#### Error Isolation

- Enhanced safety for RustyScript plugins with monitoring
- Structured error responses with error types and execution times
- Memory usage tracking for RustyScript execution

### 2. JavaScript Plugin Optimization (`javascript/mod.rs`)

#### Removed Double Threading

- **Before**: Actor → spawn_blocking → RustyScript execution
- **After**: Actor → RustyScript execution (direct)
- **Benefit**: Eliminates unnecessary thread context switching

#### Increased Timeouts

- **Before**: 1 second timeout (too short for complex JS)
- **After**: 30 second timeout (appropriate for actor system)
- **Reason**: Complex JavaScript operations need adequate time

#### Better Error Handling

- Comprehensive error reporting with stack traces
- Enhanced result validation and type checking
- Improved cleanup of temporary module files

### 3. Filter Plugin Optimization (`filter/mod.rs`)

#### Similar Improvements

- Removed unnecessary spawn_blocking
- Increased timeout from 1 to 15 seconds
- Better error handling for boolean conditions
- Support for both expression and function-style conditions

#### Enhanced Condition Wrapping

- Automatic detection of expression vs function style
- Better boolean conversion logic
- Comprehensive error reporting

### 4. Task Actor Enhancement (`task_actor.rs`)

#### RustyScript-Aware Logging

- Special logging for JavaScript and filter tasks
- Enhanced error reporting with error types
- Execution time tracking with millisecond precision

#### Improved Error Handling

- Structured error information parsing
- Error type classification (bundling, execution, timeout)
- Better debugging information

## Benefits

### Performance

- **Reduced latency**: Eliminated unnecessary thread spawning
- **Better throughput**: Actors can handle more RustyScript tasks concurrently
- **Optimal timeouts**: Each plugin type gets appropriate execution time

### Reliability

- **Reduced timeouts**: Less likely to hit 1-second JavaScript timeout
- **Better isolation**: Enhanced error handling prevents actor crashes
- **Resource monitoring**: Track memory usage for RustyScript tasks

### Debugging

- **Enhanced logging**: Clear identification of RustyScript tasks
- **Structured errors**: Error types and execution times for better diagnosis
- **Execution tracking**: Detailed timing information for performance analysis

## Configuration

### Plugin Timeouts

```rust
fn get_plugin_timeout(plugin_name: &Option<String>) -> Duration {
    match plugin_name.as_ref().map(|s| s.as_str()) {
        Some("@anything/javascript") => Duration::from_secs(60), // RustyScript needs time
        Some("@anything/filter") => Duration::from_secs(30),     // Boolean conditions
        Some("@anything/http") => Duration::from_secs(45),       // Network operations
        _ => Duration::from_secs(15), // Default for other plugins
    }
}
```

### RustyScript Runtime Options

```rust
let runtime_options = RuntimeOptions {
    timeout: Duration::from_secs(30), // Increased from 1 second
    default_entrypoint: Some("default".to_string()),
    ..Default::default()
};
```

## Migration Notes

### Before (Issues)

- JavaScript tasks frequently timed out after 1 second
- Double thread spawning caused performance overhead
- Poor error reporting made debugging difficult
- Actor system couldn't handle RustyScript tasks efficiently

### After (Improved)

- JavaScript tasks have 30-60 second timeouts
- Direct execution within actor threads
- Structured error reporting with types and timing
- Optimized for actor system architecture

## Monitoring

The system now provides enhanced monitoring for RustyScript tasks:

- **Execution time tracking**: Millisecond precision timing
- **Memory usage monitoring**: Track resource consumption
- **Error classification**: Categorize errors by type
- **Plugin-specific logging**: Clear identification of RustyScript operations

## Future Considerations

- Consider implementing RustyScript connection pooling
- Add more sophisticated memory monitoring
- Implement task priority based on complexity
- Consider async RustyScript execution if available
