use once_cell::sync::Lazy;
use std::future::Future;
use tokio::runtime::{Builder, Runtime};

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});

pub fn run_blocking<F: Future>(future: F) -> F::Output {
    TOKIO_RUNTIME.block_on(future)
}
