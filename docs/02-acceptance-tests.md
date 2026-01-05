# Acceptance Tests

## Phase 1: Virtual Monitor & Streaming

1.  **Start Provider**
    *   Command: `cargo run -p ndp-provider-mutter -- --width 1920 --height 1080` (or similar)
    *   Expected: Provider starts, connects to DBus, and creates a virtual monitor. Log output indicates DBus success and PipeWire stream creation.

2.  **Verify Settings -> Displays**
    *   Action: Open GNOME Settings -> Displays.
    *   Expected: A new "Unknown Display" or similarly named display appears.
    *   Action: Enable the display if it's not enabled by default.

3.  **Configure Display**
    *   Action: Change resolution to 1920x1080 (if not already). Change scaling. Position it relative to the main monitor.
    *   Expected: Settings apply without error.

4.  **Window Interaction**
    *   Action: Open a window (e.g., file manager, terminal). Drag it towards the new display.
    *   Expected: Mouse cursor and window move onto the virtual display area.

5.  **Start Sink (Laptop)**
    *   Command: `cargo run -p ndp-sink -- --listen 5510`
    *   Expected: A full-screen window opens showing the content of the virtual display. Mouse movements and window updates on the provider are reflected on the sink with low latency.

6.  **Disable Display**
    *   Action: In GNOME Settings -> Displays, toggle the virtual display off.
    *   Expected: The virtual monitor disappears. The sink might go black or show a "No Signal" state / disconnect message.

7.  **Re-enable Display**
    *   Action: In GNOME Settings -> Displays, toggle the virtual display on.
    *   Expected: The virtual monitor reappears. The sink (if it has reconnect logic or if restarted) shows the display again.

8.  **Stop Provider**
    *   Action: `Ctrl+C` the provider process.
    *   Expected: Virtual monitor is removed from GNOME Settings. Sink closes or shows disconnect.

9.  **Restart Provider**
    *   Action: Run the provider command again.
    *   Expected: Virtual monitor is recreated. Sink can connect again.
