use std::{net::SocketAddr, str::FromStr, sync::Arc};

use hyper::{server::conn::http1, service::service_fn};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::net::TcpListener;
use tracing::error;

use crate::{
    ApplicationArgs,
    config::application_config::ApplicationConfig,
    handler::request_handler::handle_request,
    state::application_state::ApplicationState,
    support::{TokioIo, TokioTimer},
    util::find_dir::find_available_db_directory,
};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

pub trait Applictaion
where
    Self: Send + Sync,
{
    fn generate_state(&self) -> BoxFuture<'static, Result<ApplicationState, BoxError>> {
        Box::pin(async move {
            let db_name = "jingyue.db";
            let dir = find_available_db_directory().unwrap_or(".".into());

            let db_path = dir.join(db_name);

            let options = SqliteConnectOptions::from_str(
                format!("sqlite:{}", db_path.to_str().unwrap_or_default()).as_str(),
            )?
            .create_if_missing(true) // 关键设置：如果不存在则创建
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .foreign_keys(true);

            let db = SqlitePool::connect_with(options).await?;

            // 初始化数据库操作
            // 创建表结构 TODO
            let state = ApplicationState { db };

            Ok(state)
        })
    }

    fn init_args(&self) -> Result<ApplicationArgs, BoxError> {
        Ok(ApplicationArgs::default())
    }

    fn init_log(&self) -> Result<(), BoxError> {
        let config = tracing_subscriber::fmt::format()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
                "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            ))
            .with_level(true)
            .with_target(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_thread_names(true);

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_ansi(false)
            .event_format(config)
            .init();
        Ok(())
    }

    fn bind_server(
        &self,
        config: ApplicationConfig,
        state: ApplicationState,
    ) -> BoxFuture<'static, Result<(), BoxError>> {
        Box::pin(async move {
            let ip = if config.server.local {
                [127, 0, 0, 1]
            } else {
                [0, 0, 0, 0]
            };
            let addr = SocketAddr::from((ip, config.server.port));
            let listener = TcpListener::bind(addr).await?;

            let state = Arc::new(state);
            loop {
                let (stream, _) = listener.accept().await?;
                // Use an adapter to access something implementing `tokio::io` traits as if they implement
                // `hyper::rt` IO traits.
                let io = TokioIo::new(stream);

                let state = state.clone();

                let service = service_fn(move |req| handle_request(req, state.to_owned()));

                tokio::task::spawn(async move {
                    // Handle the connection from the client using HTTP/2 with an executor and pass any
                    // HTTP requests received on that connection to the `hello` function

                    if let Err(err) = http1::Builder::new()
                        .timer(TokioTimer::new())
                        .serve_connection(io, service)
                        .await
                    {
                        error!("Error serving connection: {}", err);
                    }
                });
            }
        })
    }

    fn run() -> BoxFuture<'static, Result<(), BoxError>>
    where
        Self: Default,
    {
        Box::pin(async move {
            let application = Self::default();
            let application_args = application.init_args()?;
            let application_config = ApplicationConfig::default();
            let application_state = application.generate_state().await?;

            application.init_log()?;
            application
                .bind_server(application_config, application_state)
                .await?;

            Ok(())
        })
    }
}

#[derive(Clone, Default)]
pub struct JingyueApplication;

impl Applictaion for JingyueApplication {}
