use std::{
    collections::HashMap,
    sync::Arc,
};

use anyhow::{bail, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional},
    net::{TcpListener, TcpStream},
};
use serde::Deserialize;

use crate::util::{V3Mode, WildcardSNI};

/// ShadowTlsServer for Windows
#[derive(Clone)]
pub struct ShadowTlsServer {
    listen_addr: Arc<String>,
    target_addr: Arc<String>,
    tls_addr: Arc<TlsAddrs>,
    password: Arc<String>,
    nodelay: bool,
    fastopen: bool,
    v3: V3Mode,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TlsAddrs {
    dispatch: HashMap<String, String>,
    fallback: String,
    wildcard_sni: WildcardSNI,
}

impl TlsAddrs {
    pub fn set_wildcard_sni(&mut self, wildcard_sni: WildcardSNI) {
        self.wildcard_sni = wildcard_sni;
    }
}

impl TryFrom<&str> for TlsAddrs {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut dispatch = HashMap::new();
        let mut fallback = String::new();

        for part in value.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if part.contains(':') {
                // This is a server:port or domain:ip:port format
                if fallback.is_empty() {
                    fallback = part.to_string();
                }
                // For now, we'll use the first one as fallback
                // TODO: Implement proper parsing for domain:ip:port format
            } else {
                // This is just a domain name
                if fallback.is_empty() {
                    fallback = format!("{}:443", part);
                }
            }
        }

        if fallback.is_empty() {
            bail!("No valid TLS address found");
        }

        Ok(Self {
            dispatch,
            fallback,
            wildcard_sni: WildcardSNI::Off,
        })
    }
}

impl std::fmt::Display for TlsAddrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fallback: {}, wildcard_sni: {:?}", self.fallback, self.wildcard_sni)
    }
}

impl ShadowTlsServer {
    pub fn new(
        listen_addr: String,
        target_addr: String,
        tls_addr: TlsAddrs,
        password: String,
        nodelay: bool,
        fastopen: bool,
        v3: V3Mode,
    ) -> Self {
        Self {
            listen_addr: Arc::new(listen_addr),
            target_addr: Arc::new(target_addr),
            tls_addr: Arc::new(tls_addr),
            password: Arc::new(password),
            nodelay,
            fastopen,
            v3,
        }
    }

    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind(&*self.listen_addr).await?;
        tracing::info!("Shadow-TLS server listening on {}", self.listen_addr);

        loop {
            let (inbound, _) = listener.accept().await?;
            let server = self.clone();
            
            tokio::spawn(async move {
                if let Err(e) = server.handle_connection(inbound).await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(&self, mut inbound: TcpStream) -> Result<()> {
        if self.nodelay {
            inbound.set_nodelay(true)?;
        }

        // For now, implement a simple proxy to the target server
        // TODO: Implement proper shadow-tls protocol with TLS handshake detection
        let mut outbound = TcpStream::connect(&*self.target_addr).await?;
        if self.nodelay {
            outbound.set_nodelay(true)?;
        }

        // Simple bidirectional copy for now
        copy_bidirectional(&mut inbound, &mut outbound).await?;

        Ok(())
    }
}