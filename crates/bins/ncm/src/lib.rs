pub mod cli;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

pub async fn termination_handler() -> anyhow::Result<()> {
    let ctrl_c = tokio::signal::ctrl_c();
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl-C received, shutting down...");
        }
        _ = sigint.recv() => {
            tracing::info!("SIGINT received, shutting down...");
        }
        _ = sigterm.recv() => {
            tracing::info!("SIGTERM received, shutting down...");
        }
    }
    Ok(())
}
