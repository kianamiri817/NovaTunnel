# NovaTunnel

Next-generation secure tunneling platform with a premium desktop GUI.

## Architecture

```
Applications
      |
      |
Local SOCKS5 Proxy
127.0.0.1:1080
      |
      |
NovaTunnel Core Engine
      |
      |
Tunnel Provider Layer
      |
      |
--------------------------------
|                              |
WARP Backend              Nova Protocol
|                              |
Cloudflare Network        Custom Tunnel Network
|
Internet
```

## Features

- **Multiple Tunnel Backends**: Cloudflare WARP, Nova Protocol, WireGuard
- **Local SOCKS5 Proxy**: Works with any application
- **Security**: DNS leak protection, IPv6 leak protection, kill switch
- **Modern UI**: Premium glassmorphism design with dark mode
- **Cross-platform**: Windows, Linux, macOS

## Tech Stack

### Core Engine
- Rust with Tokio async runtime
- Modular provider architecture
- High-performance networking

### Desktop GUI
- Tauri 2
- React + TypeScript
- Modern glassmorphism UI

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 18+
- npm or yarn

### Installation

```bash
# Clone the repository
git clone https://github.com/novatunnel/novatunnel.git
cd novatunnel

# Install UI dependencies
cd ui
npm install
cd ..

# Build the application
cargo build --release
```

### Development

```bash
# Start development server
cd ui
npm run dev

# In another terminal
cargo run --package novatunnel-app
```

## Configuration

Create a `config.json` file:

```json
{
  "provider": "warp",
  "proxy_port": 1080,
  "auto_connect": false,
  "kill_switch": true,
  "dns_protection": true,
  "dns_mode": "secure",
  "custom_dns": null,
  "log_level": "info"
}
```

## Nova Protocol

NovaTunnel includes a custom tunneling protocol called Nova Protocol with:

- **Noise-style handshake**: X25519 key exchange
- **ChaCha20-Poly1305 encryption**: Modern authenticated encryption
- **Session management**: Secure session handling
- **Multiple transports**: TCP, UDP, QUIC, WebSocket

### Protocol Format

```
+----------------+
| Version        |
+----------------+
| Session ID     |
+----------------+
| Timestamp      |
+----------------+
| Flags          |
+----------------+
| Encrypted Data |
+----------------+
```

## Security Features

- Encrypted configuration storage
- DNS leak protection
- IPv6 leak protection
- Kill switch mode
- Secure session handling
- Automatic cleanup

## Platform Support

- Windows 10/11
- Linux (Ubuntu, Fedora, Arch)
- macOS (Intel, Apple Silicon)

## License

MIT License

## CI/CD

This project uses GitHub Actions for production-grade automated builds and releases with full multi-architecture support.

### Features

- **Multi-architecture builds**: Windows x64/ARM64, Linux x64/ARM64, macOS Intel/Apple Silicon
- **Automatic version injection**: Updates Cargo.toml, package.json, tauri.conf.json on tagged releases
- **Supply chain security**: SLSA provenance, build attestations, SBOM generation
- **Security auditing**: cargo-audit for vulnerabilities, cargo-deny for licenses
- **Binary verification**: Size checks, checksum verification before publish
- **Performance reports**: Binary sizes, dependency trees, audit reports
- **Categorized release notes**: Features, fixes, performance, security, docs, breaking changes
- **Recovery mechanisms**: Automatic retry, artifact preservation for 30 days
- **Code signing**: Windows (PFX) and Apple (Certificate) signing support
- **Concurrency control**: Cancels outdated builds automatically

### Automatic Builds

- **Push to main/master**: Triggers a full build on all platforms
- **Pull requests**: Triggers a build to validate changes
- **Version tags** (`v*`): Triggers a build and creates a GitHub Release
- **Manual trigger**: Use workflow_dispatch for custom builds

### Build Pipeline

1. **Prepare** - Version injection and tag detection
2. **Lint & Audit** - Formatting, clippy, security audit, license compliance
3. **Build** (6 parallel jobs) - Windows x64, Windows ARM64, Linux x64, Linux ARM64, macOS Intel, macOS Apple Silicon
4. **Reports** - Dependency tree, audit reports, license reports
5. **Verify & Publish** - Binary verification, checksum verification, SBOM generation, release creation

### Multi-Architecture Support

| Platform | Target | Runner |
|----------|--------|--------|
| Windows x64 | `x86_64-pc-windows-msvc` | `windows-latest` |
| Windows ARM64 | `aarch64-pc-windows-msvc` | `windows-latest` |
| Linux x64 | `x86_64-unknown-linux-gnu` | `ubuntu-22.04` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `ubuntu-22.04` |
| macOS Intel | `x86_64-apple-darwin` | `macos-13` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `macos-14` |

### Release Artifacts

When a version tag is pushed, the workflow automatically creates a GitHub Release with:

**Windows x64 & ARM64:**
- `NovaTunnelSetup.exe` - NSIS installer
- `*.pdb` - Debug symbols

**Linux x64 & ARM64:**
- `NovaTunnel_*.AppImage` - Portable app
- `novatunnel_*.deb` - Debian package
- `novatunnel_*.tar.gz` - Compressed archive
- `*.debug` / `*.dwp` - Debug symbols

**macOS Intel & Apple Silicon:**
- `NovaTunnel_*.dmg` - macOS disk image
- `*.dSYM` - Debug symbols

**All Platforms:**
- `SHA256SUMS.txt` - SHA256 checksums
- `SHA512SUMS.txt` - SHA512 checksums
- `sbom-*.spdx.json` - SPDX SBOM
- `sbom-*.cyclonedx.json` - CycloneDX SBOM
- Build reports (dependency tree, audit, licenses)

### Supply Chain Security

- **SLSA Provenance**: Generated for all release artifacts
- **Build Attestations**: Attached to GitHub Release
- **SBOM**: Software Bill of Materials for transparency
- **Checksums**: SHA256 and SHA512 for all binaries

### Pre-release Support

Pre-release tags automatically create draft releases:

```bash
# Pre-release (creates draft)
git tag v1.0.0-beta.1
git push origin v1.0.0-beta.1

# Stable release (creates published release)
git tag v1.0.0
git push origin v1.0.0
```

### Code Signing

**Windows:**
- `WINDOWS_CERTIFICATE`: Base64-encoded .pfx certificate
- `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

**macOS:**
- `APPLE_CERTIFICATE`: Base64-encoded .p12 certificate
- `APPLE_CERTIFICATE_PASSWORD`: Certificate password
- `APPLE_SIGNING_IDENTITY`: Signing identity
- `APPLE_ID`: Apple ID for notarization
- `APPLE_PASSWORD`: App-specific password
- `APPLE_TEAM_ID`: Apple Developer Team ID

### Creating a Release

```bash
# Tag a commit
git tag v1.0.0
git push origin v1.0.0

# The workflow will automatically:
# 1. Inject version into all config files
# 2. Lint and audit the codebase
# 3. Build optimized binaries for all 6 platforms
# 4. Verify binary integrity and checksums
# 5. Generate SBOM and provenance
# 6. Create a GitHub Release with categorized notes
# 7. Upload all binaries, checksums, and reports
```

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting a pull request.
