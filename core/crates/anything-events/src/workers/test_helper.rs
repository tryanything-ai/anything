#![cfg(not(target_os = "windows"))]
use std::fs::remove_file;
use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    internal::test_helper::{get_test_context_with_config, setup_test_directory},
    Context, Server,
};

use super::system_change::file_watcher;

#[derive(Clone)]
pub struct TestHarness<T: 'static + Clone + Sync + Send> {
    pub root_dir: PathBuf,
    pub context: Context,
    pub server: Arc<Server>,
    pub change_receiver: postage::dispatch::Receiver<T>,
}

impl<T> TestHarness<T>
where
    T: Clone + Sync + Send,
{
    pub async fn setup() -> anyhow::Result<Self> {
        let config = setup_test_directory()?;
        let context = get_test_context_with_config(config.clone()).await;
        let server = Server::new(context.clone()).await?;

        let change_receiver = server.post_office.receive_mail::<T>().await.unwrap();

        Ok(Self {
            root_dir: config.root_dir,
            context,
            server,
            change_receiver,
        })
    }

    pub async fn watch_tempdir(&mut self) -> tokio::task::JoinHandle<()> {
        // Spawn watcher thread
        let server_clone = self.server.clone();
        tokio::spawn(async move { file_watcher(server_clone).await.unwrap() })
    }

    pub fn create_flow_file(&mut self, filename: String) -> PathBuf {
        let path = self.root_dir.as_path().join("flows").join(filename);
        fs::File::create(&path).unwrap();
        path
    }

    pub fn modify_flow_file(&mut self, flowfile_path: String, contents: Option<String>) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.root_dir.join("flows").join(flowfile_path.clone()))
            .unwrap();

        let appended_contents = match contents {
            None => "hello world".to_string(),
            Some(b) => b.clone(),
        }
        .clone();

        file.write_all(appended_contents.as_bytes()).unwrap();
    }

    pub fn remove_flow_file(&mut self, flowfile_path: String) -> anyhow::Result<()> {
        let file_path = self.root_dir.join("flows").join(flowfile_path);
        fs::remove_file::<PathBuf>(file_path.into())?;
        Ok(())
    }
}
