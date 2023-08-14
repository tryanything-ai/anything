use tokio::time::{sleep, Duration};
use tauri::{
    AppHandle,  Runtime, 
};

pub async fn scheduler<R: Runtime>(app: &AppHandle<R>){
    loop {
        let app_handle = app.clone(); 

        tokio::spawn(async move {
            process(&app_handle).await;
        });

       sleep(Duration::from_secs(1)).await; 
    }
}

async fn process<R: Runtime>(_app: &AppHandle<R>) {

    println!("Processesed this time blocks tasks.");
}


// Inspiration 
// https://tokio.rs/tokio/tutorial/shared-state