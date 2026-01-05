# Media Pipelines
Provider: PipeWire -> x264enc (or vaapi) -> rtph264pay -> udpsink
Sink: udpsrc -> rtph264depay -> h264parse -> avdec_h264 (or vaapi) -> autovideosink
