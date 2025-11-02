use std::{
    sync::Arc,
    io::Cursor,
};

use anyhow::{bail, Result};
use byteorder::{BigEndian, WriteBytesExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional},
    net::{TcpListener, TcpStream},
};
use tokio_rustls::{TlsConnector, rustls::{ClientConfig, RootCertStore, ServerName}};
use rand::{seq::SliceRandom, Rng};
use serde::{de::Visitor, Deserialize};

use crate::util::{V3Mode, kdf, xor_slice, Hmac};

const FAKE_REQUEST_LENGTH_RANGE: (usize, usize) = (16, 64);

/// ShadowTlsClient for Windows
#[derive(Clone)]
pub struct ShadowTlsClient {
    listen_addr: Arc<String>,
    target_addr: Arc<String>,
    tls_connector: TlsConnector,
    tls_names: Arc<TlsNames>,
    password: Arc<String>,
    nodelay: bool,
    fastopen: bool,
    v3: V3Mode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TlsNames(Vec<ServerName>);

impl TlsNames {
    #[inline]
    pub fn random_choose(&self) -> &ServerName {
        self.0.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl TryFrom<&str> for TlsNames {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let names: Result<Vec<_>, _> = value
            .split(';')
            .map(|s| ServerName::try_from(s.trim()))
            .collect();
        Ok(Self(names?))
    }
}

impl std::fmt::Display for TlsNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self.0.iter().map(|n| format!("{:?}", n)).collect();
        write!(f, "{}", names.join(";"))
    }
}

#[derive(Clone, Debug, Default)]
pub struct TlsExtConfig {
    alpn: Option<Vec<Vec<u8>>>,
}

impl From<Option<Vec<String>>> for TlsExtConfig {
    fn from(alpn: Option<Vec<String>>) -> Self {
        Self {
            alpn: alpn.map(|v| v.into_iter().map(|s| s.into_bytes()).collect()),
        }
    }
}

impl std::fmt::Display for TlsExtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.alpn {
            Some(alpn) => {
                let alpn_strs: Vec<String> = alpn
                    .iter()
                    .map(|a| String::from_utf8_lossy(a).to_string())
                    .collect();
                write!(f, "ALPN: {}", alpn_strs.join(";"))
            }
            None => write!(f, "No ALPN"),
        }
    }
}

impl ShadowTlsClient {
    pub fn new(
        listen_addr: String,
        target_addr: String,
        tls_names: TlsNames,
        tls_ext: TlsExtConfig,
        password: String,
        nodelay: bool,
        fastopen: bool,
        v3: V3Mode,
    ) -> Result<Self> {
        let mut root_store = RootCertStore::empty();
        root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(
            |ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            },
        ));

        let mut config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        if let Some(alpn) = tls_ext.alpn {
            config.alpn_protocols = alpn;
        }

        let connector = TlsConnector::from(Arc::new(config));

        Ok(Self {
            listen_addr: Arc::new(listen_addr),
            target_addr: Arc::new(target_addr),
            tls_connector: connector,
            tls_names: Arc::new(tls_names),
            password: Arc::new(password),
            nodelay,
            fastopen,
            v3,
        })
    }

    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind(&*self.listen_addr).await?;
        tracing::info!("Shadow-TLS client listening on {}", self.listen_addr);

        loop {
            let (inbound, _) = listener.accept().await?;
            let client = self.clone();
            
            tokio::spawn(async move {
                if let Err(e) = client.handle_connection(inbound).await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(&self, mut inbound: TcpStream) -> Result<()> {
        if self.nodelay {
            inbound.set_nodelay(true)?;
        }

        // Connect to shadow-tls server
        let mut outbound = TcpStream::connect(&*self.target_addr).await?;
        if self.nodelay {
            outbound.set_nodelay(true)?;
        }

        // Perform TLS handshake with chosen server name
        let server_name = self.tls_names.random_choose().clone();
        let tls_stream = self.tls_connector.connect(server_name, outbound).await?;

        // For now, just do simple bidirectional copy
        // TODO: Implement proper shadow-tls protocol
        let (mut ri, mut wi) = inbound.split();
        let (mut ro, mut wo) = tokio::io::split(tls_stream);

        let client_to_server = async {
            tokio::io::copy(&mut ri, &mut wo).await
        };

        let server_to_client = async {
            tokio::io::copy(&mut ro, &mut wi).await
        };

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}

// Placeholder implementations for compatibility
use rustls::OwnedTrustAnchor;

impl<'de> Deserialize<'de> for TlsNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TlsNamesVisitor;

        impl<'de> Visitor<'de> for TlsNamesVisitor {
            type Value = TlsNames;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string containing TLS server names separated by semicolons")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                TlsNames::try_from(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(TlsNamesVisitor)
    }
}