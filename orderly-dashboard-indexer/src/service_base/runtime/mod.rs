use anyhow::Result;
use futures::Future;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

/// defaulted runtime threads number
const DEFAULT_THREADS: usize = 5;
const RUNTIME_CONTEXT: &str = "runtime_context";
static THREADS_NUMBER: OnceCell<usize> = OnceCell::new();

/// init_pool_workers_num, max size is thread_count
pub fn init_pool_workers_num(nums: usize) {
    let thread_count = num_cpus::get();
    let real_nums = if nums > thread_count {
        thread_count
    } else {
        nums
    };
    THREADS_NUMBER.set(real_nums).ok();
    tracing::debug!(
        "[RUNTIME]: init_thread_nums, input:{},really set num:{}",
        nums,
        real_nums
    );
}

/// runtime pool
static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    let rt = Builder::new_multi_thread()
        .worker_threads(*THREADS_NUMBER.get_or_init(|| DEFAULT_THREADS))
        .thread_name("runtime-worker")
        .thread_stack_size(512 * 1024)
        .enable_time()
        .enable_io()
        .on_thread_start(|| {
            tracing::info!("[RUNTIME]: thread {:?} start", std::thread::current().id());
        })
        .build()
        .expect("init runtime failed");
    rt
});

/// schedule futures by tokio thread pool, limit to return Result
pub fn spawn_future<F, O>(fut: F) -> JoinHandle<()>
where
    F: 'static + Future<Output = Result<O>> + Send,
    O: 'static + Send,
{
    let fut_wrapper = async {
        if let Err(err) = fut.await {
            tracing::warn!(
                target: RUNTIME_CONTEXT,
                "[RUNTIME]: future exec error:{}",
                err
            );
        }
    };
    RUNTIME.spawn(fut_wrapper)
}

/// schedule futures by tokio thread pool
pub fn raw_spawn_future<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    RUNTIME.spawn(future)
}

/// runtime pool
static LOW_PRIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("low-prio-runtime-worker")
        .thread_stack_size(512 * 1024)
        .enable_time()
        .enable_io()
        .on_thread_start(|| {
            tracing::info!(
                "[LOW-PRIO-RUNTIME]: thread {:?} start",
                std::thread::current().id()
            );
        })
        .build()
        .expect("init low pro runtime failed");
    rt
});

/// schedule futures by tokio thread pool
pub fn spawn_low_prio_future<F, O>(fut: F) -> JoinHandle<()>
where
    F: 'static + Future<Output = Result<O>> + Send,
    O: 'static + Send,
{
    let fut_wrapper = async {
        if let Err(err) = fut.await {
            tracing::warn!(
                target: RUNTIME_CONTEXT,
                "[LOW-PRIO-RUNTIME]: future exec error:{}",
                err
            );
        }
    };
    LOW_PRIO_RUNTIME.spawn(fut_wrapper)
}

#[cfg(test)]
mod tests {
    use super::spawn_future;
    async fn async_task() -> anyhow::Result<i32> {
        let res = 1;
        println!("future async_task, res={}", res);
        Ok(res)
    }
    #[test]
    fn test_async_spawn_future() {
        spawn_future(async_task());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
