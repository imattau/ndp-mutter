# NDP-Mutter (Extended Monitor via GNOME Virtual Display)

Turn a laptop into a second monitor for a GNOME desktop over LAN.

## Goal
Make a laptop behave like a second monitor for a GNOME (Mutter/Wayland) desktop, where the “monitor” is a **real virtual display** visible in **Settings → Displays**, and the laptop is a low-latency sink showing only that virtual display.

## Architecture
*   **Provider (Desktop)**: Creates a virtual monitor in Mutter via DBus, captures it via PipeWire, encodes (H.264), and streams over UDP (RTP).
*   **Sink (Laptop)**: Receives RTP/UDP, decodes, and displays full-screen.
*   **Control Plane**: Basic coordination (Phase 2).

## Repository Layout
*   `crates/ndp-provider-mutter`: The host application.
*   `crates/ndp-sink`: The client application.
*   `crates/ndp-common`: Shared protocol definitions.
*   `tools/ndp-inspect`: Diagnostics tool.
*   `docs/`: Detailed design docs and acceptance tests.

## Prerequisites
*   Rust (stable)
*   GStreamer development libraries (`libgstreamer1.0-dev`, `libgstreamer-plugins-base1.0-dev`)
*   GNOME Shell / Mutter (running on Wayland)

## Quickstart
```bash
./quickstart.sh
```

## Usage

### 1. Diagnostics (Check environment)
```bash
cargo run -p ndp-inspect
```

### 2. Start Provider (on Desktop)
Replace `<LAPTOP_IP>` with your laptop's IP address.
```bash
cargo run -p ndp-provider-mutter -- --peer <LAPTOP_IP>:5510 --width 1920 --height 1080
```

### 3. Start Sink (on Laptop)
```bash
cargo run -p ndp-sink -- --listen 5510
```

## Acceptance Tests
See `docs/02-acceptance-tests.md` for the full validation procedure.