# Shadow TLS Windows - 快速开始指南

## 🚀 快速下载和使用

### 1. 下载预编译程序

访问 [Releases 页面](https://github.com/sukyz/windows-shadow-tls/releases) 下载适合您系统的版本：

- **Windows x64**: `shadow-tls-windows-x86_64.exe`
- **Windows ARM64**: `shadow-tls-windows-aarch64.exe`
- **macOS Intel**: `shadow-tls-windows-macos-x86_64`
- **macOS Apple Silicon**: `shadow-tls-windows-macos-aarch64`
- **Linux**: `shadow-tls-windows-linux-x86_64`

### 2. Windows 使用方法

#### 服务端部署（在墙外服务器）

```cmd
# 下载并重命名文件
ren shadow-tls-windows-x86_64.exe shadow-tls.exe

# 启动服务端（以管理员身份运行CMD）
shadow-tls.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_password_here
```

#### 客户端使用（在本地Windows）

```cmd
# 启动客户端
shadow-tls.exe client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_password_here
```

### 3. 配合其他代理使用

Shadow TLS 通常需要配合加密代理使用，推荐配置：

#### 服务端配置
```cmd
# 1. 启动 Shadowsocks 服务端
ss-server.exe -s 127.0.0.1 -p 8080 -k your_ss_password -m aes-256-gcm

# 2. 启动 Shadow TLS 服务端
shadow-tls.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_shadowtls_password
```

#### 客户端配置
```cmd
# 1. 启动 Shadow TLS 客户端
shadow-tls.exe client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_shadowtls_password

# 2. 启动 Shadowsocks 客户端
ss-local.exe -s 127.0.0.1 -p 1080 -l 1081 -k your_ss_password -m aes-256-gcm

# 3. 配置浏览器代理为 127.0.0.1:1081
```

### 4. 推荐的 TLS 服务器

选择可靠的 TLS 服务器作为伪装：

- `cloudflare.com` - 稳定可靠
- `microsoft.com` - 企业环境友好
- `apple.com` - 广泛信任
- `google.com` - 全球可达
- `github.com` - 开发者友好

### 5. 常见问题解决

#### Windows 防火墙问题
```cmd
# 以管理员身份运行，添加防火墙规则
netsh advfirewall firewall add rule name="Shadow TLS" dir=in action=allow protocol=TCP localport=443
```

#### 端口占用问题
```cmd
# 检查端口占用
netstat -ano | findstr :443

# 如果443端口被占用，可以使用其他端口如8443
shadow-tls.exe server --listen 0.0.0.0:8443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_password
```

#### 权限问题
- 在Windows上监听1024以下端口需要管理员权限
- 右键点击CMD或PowerShell，选择"以管理员身份运行"

### 6. 性能优化建议

#### 服务端优化
```cmd
# 启用TCP Fast Open（如果系统支持）
shadow-tls.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_password --fastopen

# 禁用Nagle算法以降低延迟
shadow-tls.exe server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_password --disable-nodelay
```

#### 客户端优化
```cmd
# 同样的优化选项
shadow-tls.exe client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_password --fastopen --disable-nodelay
```

### 7. 安全建议

1. **使用强密码**: 密码长度至少16位，包含大小写字母、数字和特殊字符
2. **定期更换密码**: 建议每月更换一次密码
3. **选择可信的TLS服务器**: 使用大公司的域名作为伪装
4. **监控日志**: 定期检查连接日志，发现异常及时处理

### 8. 故障排除

#### 启用详细日志
```cmd
# Windows CMD
set RUST_LOG=debug
shadow-tls.exe client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_password

# PowerShell
$env:RUST_LOG="debug"
.\shadow-tls.exe client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_password
```

#### 测试连接
```cmd
# 测试到服务器的连接
telnet your_server_ip 443

# 测试本地代理
curl --proxy socks5://127.0.0.1:1081 https://www.google.com
```

### 9. 自动化部署

#### Windows 服务安装（可选）
可以使用 NSSM 将 Shadow TLS 安装为 Windows 服务：

```cmd
# 下载 NSSM
# 安装服务
nssm install ShadowTLS "C:\path\to\shadow-tls.exe"
nssm set ShadowTLS Arguments "client --listen 127.0.0.1:1080 --server your_server_ip:443 --sni cloudflare.com --password your_password"
nssm start ShadowTLS
```

### 10. 获取帮助

- **GitHub Issues**: [提交问题](https://github.com/sukyz/windows-shadow-tls/issues)
- **详细文档**: [README-WINDOWS.md](./README-WINDOWS.md)
- **原项目文档**: [Shadow TLS Wiki](https://github.com/ihciah/shadow-tls/wiki)

---

**注意**: 请确保遵守当地法律法规，合理使用网络代理工具。