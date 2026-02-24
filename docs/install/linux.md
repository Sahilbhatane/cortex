---
hide:
    - toc
---

# Installing CX Terminal on Linux

=== "APT (Ubuntu/Debian)"

    ## Using the APT Repository (Recommended)

    The easiest way to install CX Terminal on Ubuntu or Debian-based systems:

    ```bash
    # Add CX Linux repository
    curl -fsSL https://repo.cxlinux.com/key.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/cxlinux.gpg
    echo "deb [signed-by=/etc/apt/keyrings/cxlinux.gpg] https://repo.cxlinux.com/apt stable main" | sudo tee /etc/apt/sources.list.d/cxlinux.list

    # Install CX Terminal
    sudo apt update && sudo apt install cx-terminal
    ```

    ### Supported Versions
    
    | Distribution | Versions |
    |--------------|----------|
    | Ubuntu | 20.04 LTS, 22.04 LTS, 24.04 LTS |
    | Debian | 11 (Bullseye), 12 (Bookworm) |

=== "DNF (Fedora/RHEL)"

    ## Using DNF on Fedora

    ```bash
    # Add CX Linux repository
    sudo dnf config-manager --add-repo https://repo.cxlinux.com/rpm/cxlinux.repo

    # Install CX Terminal
    sudo dnf install cx-terminal
    ```

    ### Supported Versions
    
    | Distribution | Versions |
    |--------------|----------|
    | Fedora | 39, 40, 41 |
    | CentOS | Stream 9 |
    | RHEL | 9+ |

=== "Nix/NixOS"

    ## Using Nix Flakes

    CX Terminal provides a Nix flake for reproducible builds:

    ```bash
    # Try it without installing
    nix run github:cxlinux-ai/cx-core#cx-terminal

    # Install to profile
    nix profile install github:cxlinux-ai/cx-core#cx-terminal
    ```

    ### NixOS Configuration

    Add to your `flake.nix`:

    ```nix
    {
      inputs.cx-terminal.url = "github:cxlinux-ai/cx-core";
      
      outputs = { self, nixpkgs, cx-terminal }: {
        nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
          modules = [
            ({ pkgs, ... }: {
              environment.systemPackages = [
                cx-terminal.packages.${pkgs.system}.default
              ];
            })
          ];
        };
      };
    }
    ```

=== "AppImage"

    ## Using AppImage

    AppImage provides a portable, distribution-agnostic package:

    ```bash
    # Download the latest AppImage
    curl -LO https://github.com/cxlinux-ai/cx-core/releases/latest/download/cx-terminal.AppImage

    # Make it executable
    chmod +x cx-terminal.AppImage

    # Run it
    ./cx-terminal.AppImage
    ```

    For convenience, move it to your PATH:

    ```bash
    mkdir -p ~/.local/bin
    mv cx-terminal.AppImage ~/.local/bin/cx-terminal
    ```

=== "Build from Source"

    ## Building from Source

    ### Prerequisites

    Install Rust and system dependencies:

    ```bash
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
    ```

    **Ubuntu/Debian:**
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

    **Fedora:**
    ```bash
    sudo dnf install -y \
        gcc-c++ cmake pkg-config \
        fontconfig-devel freetype-devel libxcb-devel \
        libxkbcommon-devel libxkbcommon-x11-devel \
        wayland-devel openssl-devel \
        mesa-libEGL-devel mesa-libGL-devel
    ```

    **Arch:**
    ```bash
    sudo pacman -S base-devel cmake pkg-config \
        fontconfig freetype2 libxcb libxkbcommon \
        wayland openssl mesa
    ```

    ### Build

    ```bash
    git clone https://github.com/cxlinux-ai/cx-core.git
    cd cx-core
    cargo build --release
    ```

    ### Install

    ```bash
    sudo install -Dm755 target/release/wezterm /usr/bin/cx-terminal
    sudo install -Dm755 target/release/wezterm-gui /usr/bin/cx-terminal-gui
    ```

---

## Post-Installation

### Shell Integration

For the best experience, add shell integration to your shell config:

**Bash** (`~/.bashrc`):
```bash
source /usr/share/cx-terminal/shell-integration/cx.bash
```

**Zsh** (`~/.zshrc`):
```bash
source /usr/share/cx-terminal/shell-integration/cx.zsh
```

**Fish** (`~/.config/fish/config.fish`):
```fish
source /usr/share/cx-terminal/shell-integration/cx.fish
```

### Verify Installation

```bash
cx-terminal --version
```

---

## Troubleshooting

### GPU Issues

If you experience graphics issues, try software rendering:

```bash
WGPU_BACKEND=gl cx-terminal
```

### Wayland Issues

To force X11 mode:

```bash
WAYLAND_DISPLAY= cx-terminal
```

### Missing Libraries

If you see library errors, ensure all dependencies are installed using the commands in the "Build from Source" section.
