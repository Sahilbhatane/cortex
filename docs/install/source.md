## Building CX Terminal from Source

If your system isn't covered by the pre-built packages, you can build CX Terminal yourself. It runs on Linux, macOS, and Windows.

### Prerequisites

* Install `rustup` to get the Rust compiler: [Install rustup](https://www.rust-lang.org/en-US/install.html)
* Rust version 1.75 or later is required
* Git

### Quick Build

```bash
curl https://sh.rustup.rs -sSf | sh -s
git clone --depth=1 --branch=main --recursive https://github.com/cxlinux-ai/cx-core.git
cd cx-core
git submodule update --init --recursive
./get-deps
cargo build --release
```

The binaries will be in `target/release/`.

### System Dependencies

#### Ubuntu/Debian
```bash
sudo apt install -y \
    build-essential cmake pkg-config \
    libfontconfig1-dev libfreetype6-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libxcb-keysyms1-dev libxcb-icccm4-dev libxcb-image0-dev \
    libxkbcommon-dev libxkbcommon-x11-dev \
    libwayland-dev libssl-dev \
    libegl1-mesa-dev libgl1-mesa-dev
```

#### Fedora
```bash
sudo dnf install -y \
    gcc-c++ cmake pkg-config \
    fontconfig-devel freetype-devel libxcb-devel \
    libxkbcommon-devel libxkbcommon-x11-devel \
    wayland-devel openssl-devel \
    mesa-libEGL-devel mesa-libGL-devel
```

#### Arch
```bash
sudo pacman -S base-devel cmake pkg-config \
    fontconfig freetype2 libxcb libxkbcommon \
    wayland openssl mesa
```

#### macOS
```bash
xcode-select --install
brew install pkg-config fontconfig freetype openssl cmake
```

### Build Specific Components

```bash
# GUI application only
cargo build --release -p wezterm-gui

# CLI only  
cargo build --release -p wezterm

# Mux server
cargo build --release -p wezterm-mux-server
```

### Building without Wayland

On systems with X11 but no Wayland:

```bash
cargo build --release --no-default-features --features x11
```

### Building without X11

On pure Wayland systems:

```bash
cargo build --release --no-default-features --features wayland
```

### Running

```bash
# Run directly
cargo run --release --bin wezterm-gui

# Or from the build output
./target/release/wezterm-gui
```

### Installation

```bash
# System-wide
sudo install -Dm755 target/release/wezterm /usr/bin/cx-terminal
sudo install -Dm755 target/release/wezterm-gui /usr/bin/cx-terminal-gui

# Local
mkdir -p ~/.local/bin
cp target/release/wezterm ~/.local/bin/cx-terminal
cp target/release/wezterm-gui ~/.local/bin/cx-terminal-gui
```

### Troubleshooting

**zlib error**: Make sure you initialized submodules:
```bash
git submodule update --init --recursive
```

**OpenSSL not found**:
```bash
# Linux
export OPENSSL_DIR=/usr

# macOS
export OPENSSL_DIR=$(brew --prefix openssl)
```

**GPU issues**: Try software rendering:
```bash
WGPU_BACKEND=gl ./target/release/wezterm-gui
```
