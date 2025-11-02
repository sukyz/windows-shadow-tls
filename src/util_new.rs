use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};
use hmac::{Hmac as HmacImpl, Mac};
use sha2::Sha256;

pub type Hmac = HmacImpl<Sha256>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum V3Mode {
    Disabled,
    Lossy,
    Strict,
}

impl fmt::Display for V3Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            V3Mode::Disabled => write!(f, "disabled"),
            V3Mode::Lossy => write!(f, "lossy"),
            V3Mode::Strict => write!(f, "strict"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum WildcardSNI {
    Off,
    On,
    Auto,
}

impl WildcardSNI {
    pub fn from_str(s: &str, _ignore_case: bool) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "off" | "false" | "0" => Ok(WildcardSNI::Off),
            "on" | "true" | "1" => Ok(WildcardSNI::On),
            "auto" => Ok(WildcardSNI::Auto),
            _ => Err("Invalid wildcard SNI value"),
        }
    }
}

impl FromStr for WildcardSNI {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s, true)
    }
}

impl fmt::Display for WildcardSNI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WildcardSNI::Off => write!(f, "off"),
            WildcardSNI::On => write!(f, "on"),
            WildcardSNI::Auto => write!(f, "auto"),
        }
    }
}

pub fn kdf(password: &[u8], salt: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    hasher.finalize().into()
}

pub fn xor_slice(a: &mut [u8], b: &[u8]) {
    for (a_byte, b_byte) in a.iter_mut().zip(b.iter()) {
        *a_byte ^= b_byte;
    }
}

// Placeholder functions for compatibility
pub mod prelude {
    pub use anyhow::{bail, Result};
}

pub async fn resolve(addr: &str) -> anyhow::Result<std::net::SocketAddr> {
    use tokio::net::lookup_host;
    let mut addrs = lookup_host(addr).await?;
    addrs.next().ok_or_else(|| anyhow::anyhow!("No address found for {}", addr))
}

pub fn bind_with_pretty_error(_addr: &str) -> anyhow::Result<()> {
    // Placeholder implementation
    Ok(())
}

pub fn mod_tcp_conn(_stream: &tokio::net::TcpStream, _nodelay: bool, _fastopen: bool) -> anyhow::Result<()> {
    // Placeholder implementation
    Ok(())
}

pub fn support_tls13() -> bool {
    true // Assume TLS 1.3 is supported
}

pub async fn verified_relay<R, W>(_reader: R, _writer: W) -> anyhow::Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    // Placeholder implementation
    Ok(())
}

pub async fn copy_bidirectional<A, B>(a: &mut A, b: &mut B) -> anyhow::Result<(u64, u64)>
where
    A: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    B: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    tokio::io::copy_bidirectional(a, b).await.map_err(Into::into)
}

pub async fn copy_until_eof<R, W>(reader: &mut R, writer: &mut W) -> anyhow::Result<u64>
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    tokio::io::copy(reader, writer).await.map_err(Into::into)
}

pub trait CursorExt {
    // Placeholder trait
}

impl<T> CursorExt for std::io::Cursor<T> {}