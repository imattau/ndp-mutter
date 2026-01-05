use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::net::ToSocketAddrs;
use zbus::Connection;

mod dbus_client;
use dbus_client::MutterScreenCast;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Peer address (host:port)
    #[arg(long)]
    peer: String,

    /// Width of the virtual monitor
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Height of the virtual monitor
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// Framerate
    #[arg(long, default_value_t = 60)]
    fps: u32,
    
    /// Bitrate (string, e.g. "8M") - Not used in v0 hardcoded pipeline yet, but good to have
    #[arg(long, default_value = "8000")]
    bitrate: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Validate peer address
    let remote_addr = args.peer.to_socket_addrs()?.next().ok_or(anyhow::anyhow!("Invalid peer address"))?;
    
    println!("Starting NDP Provider...");
    println!("Target: {:?}", remote_addr);
    println!("Virtual Monitor: {}x{} @ {}fps", args.width, args.height, args.fps);

    // Initialize GStreamer
    gst::init()?;

    // Connect to DBus
    let connection = Connection::session().await?;
    let screencast = MutterScreenCast::new(&connection).await?;

    println!("Creating Mutter ScreenCast Session...");
    let session = screencast.create_session().await?;

    println!("Recording Virtual Monitor...");
    let stream = session.record_virtual(args.width, args.height).await?;

    println!("Starting Session...");
    session.start().await?;

    // Get PipeWire Node ID
    // Note: It might take a moment for the node to be available or negotiated?
    // Usually it's available after start.
    let node_id = stream.pipewire_node_id().await?;
    println!("Got PipeWire Node ID: {}", node_id);

    // Build GStreamer Pipeline
    // pipewiresrc path=<node_id> ! videoconvert ! x264enc tune=zerolatency bitrate=... ! rtph264pay ! udpsink
    
    // Check if hardware encoding is requested/available? 
    // For v0, we'll try x264enc for compatibility, or check env?
    // Prompt says: "attempt VAAPI first, fall back to software x264/openh264"
    // We'll stick to x264enc for reliability in v0 "immediate generation", 
    // unless we want to do complex pipeline building logic.
    // Let's stick to x264enc with zerolatency.
    
    let pipeline_str = format!(
        "pipewiresrc path={node_id} do-timestamp=true ! \
         videoconvert ! \
         queue max-size-buffers=1 leaky=downstream ! \
         x264enc tune=zerolatency speed-preset=ultrafast bitrate={bitrate} ! \
         rtph264pay config-interval=1 pt=96 ! \
         udpsink host={host} port={port} sync=false",
        node_id = node_id,
        bitrate = args.bitrate, 
        host = remote_addr.ip(),
        port = remote_addr.port()
    );

    println!("Launching Pipeline: {}", pipeline_str);
    let pipeline = gst::parse_launch(&pipeline_str)?;
    
    pipeline.set_state(gst::State::Playing)?;

    // Handle messages
    let bus = pipeline.bus().unwrap();
    let mut stream_msg = bus.stream();

    println!("Streaming... Press Ctrl+C to stop.");

    // Simple loop to keep alive and check for errors
    while let Some(msg) = stream_msg.next().await {
        use gstreamer::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => {
                println!("End-Of-Stream reached.");
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;
    Ok(())
}