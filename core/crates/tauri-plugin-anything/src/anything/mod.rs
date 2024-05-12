use crate::{AnythingState, Error};

pub mod flows;
pub use flows::*;
// pub mod persistence;
// pub use persistence::*;

#[tauri::command]
pub fn initialize() {
    println!("In initialize for anything tauri plugin");
}

#[tauri::command]
pub async fn setup() {}

#[tauri::command]
pub async fn stop(state: tauri::State<'_, AnythingState>) -> Result<(), Error> {
    let stop_tx = state.stop_tx.as_ref().unwrap();
    stop_tx.send(()).await.unwrap();
    Ok(())
}
