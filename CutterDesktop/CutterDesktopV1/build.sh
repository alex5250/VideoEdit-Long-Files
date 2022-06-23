sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
cd ./CutterDesktop/CutterDesktopV1
npm install
npm run tauri build
cd ..
cp -r  ./src-tauri/target/release/bundle ./bin

