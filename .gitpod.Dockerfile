FROM gitpod/workspace-full-vnc

# Install dependencies for Tauri and the X11 server
RUN sudo apt update && sudo apt install -y libwebkit2gtk-4.0-dev \
    build-essential curl wget libssl-dev libgtk-3-dev libappindicator3-dev \
    patchelf librsvg2-dev libappindicator-dev && \
    sudo rm -rf /var/lib/apt/lists/*

# Install Node.js 16.x and Yarn 1.x
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
RUN sudo apt install -y nodejs && sudo npm install -g yarn

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
