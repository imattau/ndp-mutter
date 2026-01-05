use std::process::Command;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NDP Mutter Inspector");
    println!("====================");

    // 1. GNOME Version
    match Command::new("gnome-shell").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("GNOME Shell: {}", version);
        }
        Err(_) => println!("GNOME Shell: Not found or error running gnome-shell"),
    }

    // 2. Mutter DBus
    println!("Checking Mutter DBus interfaces...");
    let connection = Connection::session().await?;
    
    // Check for ScreenCast interface
    let proxy = zbus::Proxy::new(
        &connection,
        "org.gnome.Mutter.ScreenCast",
        "/org/gnome/Mutter/ScreenCast",
        "org.freedesktop.DBus.Introspectable",
    )
    .await;

    match proxy {
        Ok(proxy) => {
             match proxy.call::<&str, (), String>("Introspect", &()).await {
                Ok(_) => println!("  org.gnome.Mutter.ScreenCast: FOUND"),
                Err(e) => println!("  org.gnome.Mutter.ScreenCast: ERROR calling Introspect ({})", e),
            }
        },
        Err(e) => println!("  org.gnome.Mutter.ScreenCast: NOT FOUND ({})", e),
    }

    // 3. GStreamer & PipeWire (via CLI)
    println!("Checking GStreamer plugins (via gst-inspect-1.0)...");
    let plugins = vec![
        "pipewiresrc",
        "rtph264pay",
        "rtph264depay",
        "udpsink",
        "udpsrc",
        "x264enc",
        "avdec_h264",
        "vaapih264enc", // Hardware
        "vaapih264dec", // Hardware
    ];

    for plugin in plugins {
        match Command::new("gst-inspect-1.0").arg(plugin).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("  {}: AVAILABLE", plugin);
                } else {
                    println!("  {}: MISSING", plugin);
                }
            },
            Err(_) => println!("  {}: ERROR (gst-inspect-1.0 not found?)", plugin),
        }
    }

    Ok(())
}
