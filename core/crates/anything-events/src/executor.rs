use std::{
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::runtime::{Builder, Runtime};
use tracing::error;

use crate::EvtResult;

#[allow(unused)]
pub fn build_runtime() -> EvtResult<Arc<Runtime>> {
    let raw_runtime = Builder::new_multi_thread()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("scheduler-{}", id)
        })
        .thread_stack_size(4 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("Init Tokio runtime failed.");

    let arc_runtime = Arc::new(raw_runtime);

    Ok(arc_runtime)
}

/// execute a future and if it returns (Ok or Err) then crash
pub fn spawn_or_crash<F, C, Fut>(name: impl Into<String>, ctx: C, func: F)
where
    F: Fn(C) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
    C: Send + Sync + 'static,
{
    let name = name.into();

    let _ = tokio::spawn(async move {
        match func(ctx).await {
            Ok(_) => unreachable!("func never returns"),
            Err(err) => error!("task {} failed: {:?}", name, err),
        }
        error!("task {} failed, aborting!", name);
        std::process::exit(1);
    });
}
