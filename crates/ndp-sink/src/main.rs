use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(long, default_value_t = 5510)]
    listen: u16,

    /// Start fullscreen (Best effort for v0)
    #[arg(long, default_value_t = false)]
    fullscreen: bool,

    /// Disable audio (ignored for now as we only do video)
    #[arg(long, default_value_t = true)]
    no_audio: bool,

    /// Show stats
    #[arg(long, default_value_t = false)]
    stats: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("Starting NDP Sink...");
    println!("Listening on port: {}", args.listen);

    // Initialize GStreamer
    gst::init()?;

    // Build pipeline
    // udpsrc -> rtph264depay -> h264parse -> avdec_h264 -> autovideosink
    // We need strict caps on udpsrc for it to negotiate with depayloader
    let pipeline_str = format!(
        "udpsrc port={} caps=\"application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264, payload=(int)96\" ! \
         rtph264depay ! h264parse ! avdec_h264 ! \
         videoconvert ! autovideosink sync=false",
        args.listen
    );

    println!("Launching Pipeline: {}", pipeline_str);
    let pipeline = gst::parse_launch(&pipeline_str)?;

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    pipeline.set_state(gst::State::Playing)?;

    // Main loop
    let bus = pipeline.bus().unwrap();
    
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
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
            MessageView::StateChanged(s) => {
                if s.src().map(|s| s == pipeline).unwrap_or(false) {
                     println!("Pipeline state changed from {:?} to {:?}", s.old(), s.current());
                }
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;
    Ok(())
}