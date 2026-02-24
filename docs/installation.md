---
hide:
  - navigation
---

# Installation

CX Terminal is available for major platforms. Choose your preferred installation method:

## Quick Install

### Linux (APT)
```bash
curl -fsSL https://repo.cxlinux.com/key.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/cxlinux.gpg
echo "deb [signed-by=/etc/apt/keyrings/cxlinux.gpg] https://repo.cxlinux.com/apt stable main" | sudo tee /etc/apt/sources.list.d/cxlinux.list
sudo apt update && sudo apt install cx-terminal
```

### macOS (Homebrew)
```bash
brew tap cxlinux-ai/tap
brew install cx-terminal
```

## Detailed Guides

- [Linux](install/linux.md) - Ubuntu, Debian, Fedora, Arch, NixOS
- [macOS](install/macos.md) - Homebrew, DMG, Build from Source
- [Build from Source](INSTALL.md) - Full build instructions

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| OS | Linux kernel 4.15+ / macOS 11+ | Latest LTS |
| GPU | OpenGL 3.3 | Vulkan support |
| RAM | 256 MB | 512 MB |
| Disk | 100 MB | 200 MB |

## Supported Platforms

| Platform | Versions |
|----------|----------|
| **Ubuntu** | 20.04, 22.04, 24.04 |
| **Debian** | 11, 12 |
| **Fedora** | 39, 40, 41 |
| **CentOS** | Stream 9 |
| **macOS** | 11+ (Intel & Apple Silicon) |
