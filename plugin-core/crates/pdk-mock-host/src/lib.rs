use extism::*;

// pretend this is redis or something :)
type KVStore = std::collections::BTreeMap<String, Vec<u8>>;

// When a first argument separated with a semicolon is provided to `host_fn` it is used as the
// variable name and type for the `UserData` parameter
host_fn!(kv_read(user_data: KVStore; key: String) -> u32 {
    let kv = user_data.get()?;
    let kv = kv.lock().unwrap();
    let value = kv
        .get(&key)
        .map(|x| u32::from_le_bytes(x.clone().try_into().unwrap()))
        .unwrap_or_else(|| 0u32);
    Ok(value)
});

host_fn!(kv_write(user_data: KVStore; key: String, value: u32) {
    let kv = user_data.get()?;
    let mut kv = kv.lock().unwrap();
    kv.insert(key, value.to_le_bytes().to_vec());
    Ok(())
});

fn main() {}
