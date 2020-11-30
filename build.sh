set -e

cargo build

./macos_package.sh Plugin target/debug/libvst.dylib

cp -R ./Plugin.vst ~/Library/Audio/Plug-Ins/VST/