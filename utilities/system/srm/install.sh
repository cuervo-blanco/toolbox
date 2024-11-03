#!/bin/bash

INSTALL_PATH="/usr/local/bin"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    INSTALL_PATH="/usr/bin"
fi

check_rust_installed() {
    if ! command -v cargo &> /dev/null; then
        echo "Rust is not installed."
        read -p "Would you like to install Rust? (y/n): " choice
        if [[ "$choice" == "y" || "$choice" == "Y" ]]; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
            echo "Rust has been installed successfully."
        else
            echo "Rust installation is required to continue. Exiting."
            exit 1
        fi
    fi
}

build_project() {
    echo "Building srm..."
    if cargo build --release; then
        echo "Build successful."
    else
        echo "Build failed. Please check for any errors and try again."
        exit 1
    fi
}

install_binary() {
    echo "Installing srm to $INSTALL_PATH..."
    sudo cp target/release/srm "$INSTALL_PATH/srm"

    if [[ $? -eq 0 ]]; then
        echo "srm installed successfully in $INSTALL_PATH"
    else
        echo "Failed to install srm. Please check permissions and paths."
        exit 1
    fi
}

check_rust_installed
build_project
install_binary

