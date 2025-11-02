# Shadow TLS for Windows

A Windows-compatible version of [Shadow TLS](https://github.com/ihciah/shadow-tls) - a proxy to expose real TLS handshake to the firewall.

## Overview

This is a Windows-native port of the original Shadow TLS project, which was Linux-only due to its dependency on Monoio (io_uring). This version uses Tokio instead, making it fully compatible with Windows, macOS, and Linux.

**Key Features:**
- ✅ **Windows Native Support** - Runs natively on Windows without WSL
- ✅ **Cross-Platform** - Works on Windows, macOS, and Linux
- ✅ **Real TLS Handshake** - Uses legitimate certificates from trusted domains
- ✅ **No Certificate Management** - No need to sign your own certificates
- ✅ **Tokio-based** - Modern async runtime with excellent performance

## How it Works

Like the original Shadow TLS, this proxy:
1. Performs real TLS handshakes with legitimate servers (like cloudflare.com, google.com, etc.)
2. Uses their valid certificates to fool deep packet inspection
3. Tunnels your actual traffic through the established TLS connection
4. Appears as normal HTTPS traffic to firewalls and monitoring systems

The firewall sees **real** TLS handshake with **valid certificates** that you choose.

## Installation

### Option 1: Download Pre-built Binary (Recommended)

1. Download the latest Windows binary from the [Releases page](https://github.com/sukyz/windows-shadow-tls/releases)
2. Extract the executable to a folder of your choice
3. Run from Command Prompt or PowerShell

### Option 2: Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (latest stable version)
- Git

**Build Steps:**
```bash
git clone https://github.com/sukyz/windows-shadow-tls.git
cd windows-shadow-tls
cargo build --release
```

The binary will be available at `target/release/shadow-tls-windows.exe`

## Usage

### Basic Usage

**Server Side:**
```bash
shadow-tls-windows.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls example.com --password mypassword
```

**Client Side:**
```bash
shadow-tls-windows.exe client --listen 127.0.0.1:1080 --server your-server-ip:443 --sni example.com --password mypassword
```

### Command Line Options

#### Server Mode
```bash
shadow-tls-windows.exe server --help
```

Key options:
- `--listen`: Address to listen on (e.g., `0.0.0.0:443`)
- `--server`: Your actual service address (e.g., `127.0.0.1:8080`)
- `--tls`: TLS server address for handshake (e.g., `example.com:443`)
- `--password`: Password for authentication

#### Client Mode
```bash
shadow-tls-windows.exe client --help
```

Key options:
- `--listen`: Local listen address (e.g., `127.0.0.1:1080`)
- `--server`: Shadow TLS server address
- `--sni`: Server Name Indication for TLS handshake
- `--password`: Password for authentication
- `--alpn`: Application-Layer Protocol Negotiation (optional)

### Example Setup

**Scenario:** Bypass firewall using Cloudflare's certificate

1. **Server (outside firewall):**
   ```bash
   # Run your actual service (e.g., Shadowsocks)
   ss-server -s 127.0.0.1 -p 8080 -k mykey -m aes-256-gcm
   
   # Run Shadow TLS server
   shadow-tls-windows.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password mypassword
   ```

2. **Client (inside firewall):**
   ```bash
   # Run Shadow TLS client
   shadow-tls-windows.exe client --listen 127.0.0.1:1080 --server your-server-ip:443 --sni cloudflare.com --password mypassword
   
   # Configure your applications to use 127.0.0.1:1080 as proxy
   ```

## Configuration Tips

### Choosing TLS Servers

Good choices for `--tls` and `--sni`:
- `cloudflare.com` - Reliable and widely trusted
- `microsoft.com` - Good for corporate environments
- `apple.com` - Popular and trusted
- `google.com` - Widely accessible

### Performance Tuning

- Use `--disable-nodelay` if you experience connection issues
- Consider `--fastopen` for better performance (if supported)
- For high-traffic scenarios, you may want to run multiple instances

### Security Considerations

- Use strong, unique passwords
- Consider using v3 protocol with `--v3` flag for enhanced security
- Regularly update to the latest version
- Monitor logs for suspicious activity

## Differences from Original

This Windows port has the following changes:

### Runtime Changes
- **Monoio → Tokio**: Replaced Linux-specific io_uring runtime with cross-platform Tokio
- **Single-threaded**: Simplified from multi-threaded to single-threaded async model
- **Standard I/O**: Uses standard Rust async I/O traits instead of Monoio's specialized traits

### Dependency Changes
- `monoio` → `tokio`
- `monoio-rustls` → `tokio-rustls`
- `rustls-fork` → `rustls` (standard version)

### Compatibility
- ✅ **Windows 10/11** (native support)
- ✅ **Windows Server 2019/2022**
- ✅ **macOS** (Intel and Apple Silicon)
- ✅ **Linux** (all distributions)

## Troubleshooting

### Common Issues

**"Permission denied" on Windows:**
- Run Command Prompt or PowerShell as Administrator
- Check Windows Firewall settings
- Ensure the port is not already in use

**Connection timeouts:**
- Verify server and client passwords match
- Check firewall rules on both ends
- Ensure the TLS server (e.g., cloudflare.com) is accessible

**High CPU usage:**
- This is normal for TLS handshakes
- Consider using fewer concurrent connections
- Monitor system resources

### Debug Mode

Enable verbose logging:
```bash
RUST_LOG=debug shadow-tls-windows.exe client --listen 127.0.0.1:1080 --server your-server:443 --sni cloudflare.com --password mypassword
```

## Contributing

This is a community-maintained Windows port. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test on Windows
5. Submit a pull request

## License

This project maintains the same license as the original Shadow TLS project.

## Acknowledgments

- Original [Shadow TLS](https://github.com/ihciah/shadow-tls) by ihciah
- [Tokio](https://tokio.rs/) async runtime
- [rustls](https://github.com/rustls/rustls) TLS implementation

## Support

- **Issues**: [GitHub Issues](https://github.com/sukyz/windows-shadow-tls/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sukyz/windows-shadow-tls/discussions)
- **Original Project**: [Shadow TLS Wiki](https://github.com/ihciah/shadow-tls/wiki)