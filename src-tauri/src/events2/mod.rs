use tokio::time::{sleep, Duration};

pub async fn scheduler() {
    loop {

        tokio::spawn(async move {
            process().await;
        });

       sleep(Duration::from_secs(1)).await; 
    }
}

async fn process() {
    println!("Processesed this time blocks tasks.");
}


// Inspiration 
// https://tokio.rs/tokio/tutorial/shared-state