# Windows 兼容性指南

## 问题描述

如果您在Windows 10上运行Shadow TLS时遇到"与64位Windows不兼容"的错误，这通常是由以下原因造成的：

## 常见原因和解决方案

### 1. 架构不匹配
**问题**: 下载了错误的架构版本
**解决方案**: 
- 确保下载 `shadow-tls-windows-x86_64.exe` (64位版本)
- 检查您的系统架构：在命令提示符中运行 `echo %PROCESSOR_ARCHITECTURE%`
- 应该显示 `AMD64` 表示64位系统

### 2. 损坏的下载文件
**问题**: 下载过程中文件损坏
**解决方案**:
- 重新下载二进制文件
- 检查文件大小是否正确（应该在4-6MB左右）
- 尝试从不同的网络环境下载

### 3. 安全软件阻止
**问题**: Windows Defender或其他安全软件阻止执行
**解决方案**:
- 将文件添加到Windows Defender排除列表
- 临时禁用实时保护进行测试
- 检查安全软件的隔离区

### 4. 缺少运行时库
**问题**: 缺少Visual C++运行时库
**解决方案**:
- 安装 [Microsoft Visual C++ Redistributable](https://docs.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)
- 选择x64版本

### 5. 权限问题
**问题**: 没有足够的权限执行文件
**解决方案**:
- 右键点击文件 → "以管理员身份运行"
- 或者将文件移动到用户目录（如桌面）

## 兼容性检查工具

我们提供了一个PowerShell脚本来检查兼容性：

```powershell
# 下载兼容性检查脚本
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/sukyz/windows-shadow-tls/main/check-windows-compat.ps1" -OutFile "check-windows-compat.ps1"

# 运行检查
.\check-windows-compat.ps1 -BinaryPath ".\shadow-tls-windows-x86_64.exe"
```

## 系统要求

### 最低要求
- Windows 10 x64 (Build 1809或更高)
- 4GB RAM
- 50MB 可用磁盘空间

### 推荐配置
- Windows 10/11 x64 (最新版本)
- 8GB RAM
- SSD存储

## 手动验证步骤

### 1. 检查文件属性
```cmd
dir shadow-tls-windows-x86_64.exe
```
文件大小应该在4-6MB之间

### 2. 检查文件类型
```powershell
Get-ItemProperty .\shadow-tls-windows-x86_64.exe | Select-Object Name, Length, LastWriteTime
```

### 3. 测试执行
```cmd
shadow-tls-windows-x86_64.exe --version
```
应该显示版本信息而不是错误

## 构建自己的版本

如果预编译版本不工作，您可以自己编译：

### 前提条件
1. 安装 [Rust](https://rustup.rs/)
2. 确保使用MSVC工具链：`rustup default stable-x86_64-pc-windows-msvc`

### 编译步骤
```powershell
# 克隆仓库
git clone https://github.com/sukyz/windows-shadow-tls.git
cd windows-shadow-tls

# 运行构建脚本
.\build-windows.ps1
```

或者手动编译：
```cmd
# 添加目标平台
rustup target add x86_64-pc-windows-msvc

# 编译
cargo build --release --target x86_64-pc-windows-msvc

# 二进制文件位置
target\x86_64-pc-windows-msvc\release\shadow-tls-windows.exe
```

## 故障排除

### 错误: "应用程序无法正常启动(0xc000007b)"
这通常表示32位/64位混合问题：
- 确保下载64位版本
- 安装64位Visual C++运行时库

### 错误: "找不到MSVCR120.dll"
缺少Visual C++运行时库：
- 安装Microsoft Visual C++ 2013 Redistributable
- 安装Microsoft Visual C++ 2015-2022 Redistributable

### 错误: "Windows无法访问指定设备、路径或文件"
权限或安全软件问题：
- 以管理员身份运行
- 检查文件是否被阻止（右键→属性→解除阻止）
- 添加到安全软件白名单

## 获取帮助

如果问题仍然存在：

1. **GitHub Issues**: https://github.com/sukyz/windows-shadow-tls/issues
2. **提供以下信息**:
   - Windows版本 (`winver`)
   - 系统架构 (`echo %PROCESSOR_ARCHITECTURE%`)
   - 错误消息截图
   - 兼容性检查脚本的输出

## 已知兼容的系统

✅ **测试通过**:
- Windows 10 Pro x64 (Build 19041+)
- Windows 11 Pro x64
- Windows Server 2019 x64
- Windows Server 2022 x64

⚠️ **可能需要额外配置**:
- Windows 10 Home x64 (较旧版本)
- Windows Server 2016 x64

❌ **不支持**:
- Windows 7/8/8.1
- 32位Windows系统
- Windows RT/ARM (除非使用ARM64版本)