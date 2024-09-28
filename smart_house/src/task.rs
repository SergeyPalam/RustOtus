use tokio_util::sync::CancellationToken;
use tokio::task::JoinHandle;
use tokio::select;
pub struct TaskBeacon {
    pub tx_kill_token: CancellationToken,
    pub join_handle: JoinHandle<()>,
}

pub trait Start {
    fn start(self) -> impl std::future::Future<Output = ()> + Send;
}

pub fn start<T: Start + Send + 'static>(task: T) -> TaskBeacon {
    let tx_kill_token = CancellationToken::new();
    let rx_kill_token = tx_kill_token.clone();
    let join_handle = tokio::spawn(async move{
        select! {
            _ = rx_kill_token.cancelled() => {}
            _ = task.start() => {}
        }
    });
    TaskBeacon {
        tx_kill_token,
        join_handle,
    }
}

