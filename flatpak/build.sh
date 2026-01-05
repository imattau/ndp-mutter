#!/bin/bash
set -e

echo "Ensuring Flatpak runtimes are installed..."
flatpak install -y flathub org.gnome.Platform//46 org.gnome.Sdk//46 org.freedesktop.Sdk.Extension.rust-stable//23.08

REPO_ROOT=$(pwd)/..

echo "Building Provider..."
# We use --share=network to allow cargo to fetch crates
flatpak-builder --force-clean --user --install --share=network \
    build-dir-provider \
    com.github.ndp_mutter.Provider.yml

echo "Building Sink..."
flatpak-builder --force-clean --user --install --share=network \
    build-dir-sink \
    com.github.ndp_mutter.Sink.yml

echo "Done!"
echo "To run Provider: flatpak run com.github.ndp_mutter.Provider --help"
echo "To run Sink:     flatpak run com.github.ndp_mutter.Sink --help"
