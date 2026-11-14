# Supported platforms

## Release targets (v0.1.0)
| Distribution | Versions | Architecture | Status |
|---|---|---|---|
| Debian | 12 (bookworm), 13 (trixie) | amd64 (arm64 later) | 🗓 planned |
| Ubuntu | 22.04 LTS, 24.04 LTS (and the current interim release) | amd64 (arm64 later) | 🗓 planned |
| Any distribution with Flatpak | — | amd64 | 🗓 planned |

Ubuntu 22.04 support depends on the GPUI spike (P0-A): GPUI's Vulkan and Wayland requirements may need newer Mesa or system libraries than 22.04 ships. The spike confirms or drops it.

## Runtime requirements
- A GPU with Vulkan 1.2+ drivers (Mesa `radv`, `anv`, NVIDIA proprietary). Software Vulkan (`lavapipe`) works for testing but is slow.
- Wayland or X11 session.
- A Secret Service provider (GNOME Keyring, KWallet ≥ 5.97 with Secret Service, KeePassXC) for saved passwords; otherwise NineSQL uses its encrypted vault.

## Build requirements (Debian / Ubuntu)
The exact list is finalized in P0-A. Starting point for GPUI on Linux:
```bash
sudo apt install build-essential pkg-config clang cmake libvulkan-dev libwayland-dev libxkbcommon-dev libxkbcommon-x11-dev libx11-xcb-dev libxcb1-dev libfontconfig-dev libfreetype-dev libzstd-dev libasound2-dev
```

## Development only
- **macOS** (Apple Silicon and Intel): builds are kept compiling so contributors on Macs can iterate on the UI (GPUI renders with Metal). Not a release target; Linux-specific parts (Secret Service, Flatpak, `.deb`) are tested on Linux only.
- **Windows**: not supported.
