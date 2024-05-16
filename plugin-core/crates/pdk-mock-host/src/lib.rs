use extism_pdk::{Memory};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Log {
    pub time: String,
    pub message: String,
}

#[no_mangle]
pub extern "C" fn host_log(log_ptr: i64) -> i64 {
    // Find the memory at the given pointer
    let log_mem = Memory::find(log_ptr as u64).expect("can't find log memory");

    // Convert memory to a string
    let log_msg = log_mem.to_string().expect("bad data?");

    // Create a log structure or message
    let new_log = Log {
        time: "2021-09-01".to_string(),
        message: log_msg,
    };

    // Serialize the log to JSON
    let output = serde_json::to_vec(&new_log).expect("serialization failed");

    // Create new memory for the output
    let output_mem = Memory::from_bytes(&output);

    // Return the offset of the new memory as an i64
    return output_mem.expect("cant return offset").offset() as i64;
}
