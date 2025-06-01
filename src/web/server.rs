//! Web u670du52a1u5668u5b9eu73b0

use crate::core::config::Config;
use axum::Router;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Web u670du52a1u5668
pub struct Server {
    /// u914du7f6eu9879
    config: Config,
    /// u670du52a1u5668u5730u5740
    address: SocketAddr,
    /// u8def u5f84
    router: Option<Router>,
}

impl Server {
    /// u521bu5efau65b0u7684u670du52a1u5668u5b9eu4f8b
    pub fn new(config: Config, host: &str, port: u16) -> Self {
        let addr = format!("{host}:{port}").parse().expect("Invalid address");

        Self {
            config,
            address: addr,
            router: None,
        }
    }

    /// u8bbeu7f6eu8def u7531
    pub fn with_router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    /// u8fd0u884cu670du52a1u5668
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let router = self.router.clone().unwrap_or_else(|| {
            // u521bu5efau9ed8u8ba4u8def u7531
            super::routes::create_routes(&self.config)
        });

        println!("Server starting at http://{}", self.address);

        let listener = TcpListener::bind(&self.address).await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        axum::serve(listener, router.into_make_service())
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
