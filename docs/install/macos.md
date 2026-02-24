## Installing CX Terminal on macOS

CX Terminal supports macOS 11 (Big Sur) and later, with native support for both Apple Silicon and Intel Macs.

## Homebrew (Recommended)

The easiest way to install CX Terminal on macOS:

```bash
# Add the CX Linux tap
brew tap cxlinux-ai/tap

# Install CX Terminal
brew install cx-terminal
```

To upgrade:
```bash
brew upgrade cx-terminal
```

## Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/cxlinux-ai/cx-core/releases)
2. Extract the `.zip` file
3. Drag `CX-Terminal.app` to your `Applications` folder
4. Right-click and select **Open** the first time (to bypass Gatekeeper)

### Adding to PATH

To use `cx-terminal` from your shell, add to your `~/.zshrc`:

```bash
PATH="$PATH:/Applications/CX-Terminal.app/Contents/MacOS"
export PATH
```

## Build from Source

### Prerequisites

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew dependencies
brew install pkg-config fontconfig freetype openssl cmake
```

### Build

```bash
# Clone the repository
git clone https://github.com/cxlinux-ai/cx-core.git
cd cx-core

# Build release binary
cargo build --release

# The binary will be at target/release/wezterm-gui
```

### Create App Bundle

```bash
# Build the macOS app bundle
cargo build --release -p wezterm-gui
./ci/macos-bundle.sh
```

The app bundle will be created in `target/release/`.

## Verification

```bash
# Check version
cx-terminal --version

# Open from terminal
open -a CX-Terminal
```

## Troubleshooting

### "CX-Terminal can't be opened" error

If macOS blocks the app:
1. Go to **System Preferences → Security & Privacy → General**
2. Click **Open Anyway** next to the CX Terminal message

### GPU Issues

For graphics problems, try software rendering:
```bash
WGPU_BACKEND=gl /Applications/CX-Terminal.app/Contents/MacOS/cx-terminal
```
