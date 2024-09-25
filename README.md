# PlingDing

This is a Rust application that sends push notifications via pushover.net. It can be used from the command line to send messages with optional priority and image attachments.

## Prerequisites

1. Rust and Cargo installed on your system (for building from source)
2. A Pushover account with an API token and user key

## Setup

1. Clone this repository
2. Navigate to the project directory
3. Set up environment variables for your Pushover API token and user key:

   For Bash/Zsh:
   ```
   export PUSHOVER_API_TOKEN="your_api_token_here"
   export PUSHOVER_USER_KEY="your_user_key_here"
   ```

   For Fish:
   ```
   set -x PUSHOVER_API_TOKEN "your_api_token_here"
   set -x PUSHOVER_USER_KEY "your_user_key_here"
   ```

   You can add these lines to your shell's configuration file (e.g., `.bashrc`, `.zshrc`, or `config.fish`) to make them permanent.

## Building from Source

To build the application for your current platform, run:

```
cargo build --release
```

The compiled binary will be available in `target/release/plingding`.

For an optimized and smaller binary, you can use the `strip` command after building:

```
strip target/release/plingding
```

This will remove debug symbols from the binary, significantly reducing its size.

## Cross-Platform Compilation

PlingDing supports cross-platform compilation for Linux and Windows from a Linux host. To build for these platforms, follow these steps:

1. Install the `cross` tool:
   ```
   cargo install cross
   ```

2. Install the required target:
   ```
   rustup target add x86_64-pc-windows-gnu
   ```

3. Run the build script:
   ```
   chmod +x build.sh
   ./build.sh
   ```

This will create binaries for Linux and Windows in the `releases` directory.

## Creating Releases

Follow these steps:

1. Update the version number in `Cargo.toml` and `PKGBUILD`.
2. Commit all changes and push to the repository.
3. Create a new git tag for the release:
   ```
   git tag -a v0.x.x -m "Release v0.x.x"
   ```
4. Push the tag:
   ```
   git push origin v0.x.x
   ```
5. Run the build script on a Linux system to create binaries for Linux and Windows:
   ```
   ./build.sh
   ```
6. Build the macOS version on a macOS system as described in the "Building for macOS" section.
7. Create a new release on GitHub, upload the binaries for all three platforms, and publish the release.

## Installing from AUR (Arch User Repository)

If you're using an Arch-based Linux distribution, you can install PlingDing from the AUR using one of the following methods:

### Using an AUR Helper

1. Make sure you have an AUR helper installed (e.g., yay, paru)
2. Run the following command:

   ```
   yay -S plingding
   ```

   Replace `yay` with your preferred AUR helper if different.

3. Follow the prompts to install the package

### Manual Installation

If you prefer not to use an AUR helper, you can manually install plingding from the AUR:

1. Make sure you have the necessary tools installed:
   ```
   sudo pacman -S --needed base-devel git
   ```

2. Clone the AUR package repository:
   ```
   git clone https://aur.archlinux.org/plingding.git
   cd plingding
   ```

3. Build and install the package:
   ```
   makepkg -si
   ```

   This command will download the source, build the package, and install it. The `-si` flags mean it will install any needed dependencies and then install the built package.

4. If you want to review the PKGBUILD before building, you can open it in a text editor and then run `makepkg -si` when you're ready to proceed.

After installation using either method, the `plingding` command will be available system-wide.

## Usage

```
plingding [OPTIONS] --message <MESSAGE>
```

Options:
- `-m, --message <MESSAGE>`: The message to send (required)
- `-p, --priority <PRIORITY>`: The priority of the message (-2 to 2, default: 0)
- `-i, --image <IMAGE>`: The path to an image to attach (optional)

## Examples

1. Send a simple message:
   ```
   plingding -m "Hello, World!"
   ```

2. Send a message with high priority:
   ```
   plingding -m "Urgent message" -p 2
   ```

3. Send a message with an attached image:
   ```
   plingding -m "Check out this image" -i /path/to/image.jpg
   ```

## Note

Make sure to set the PUSHOVER_API_TOKEN and PUSHOVER_USER_KEY environment variables before running the application. If these environment variables are not set, the application will return an error message indicating which variable is missing.

## Platform-Specific Notes

- **Linux**: The application should work out of the box on most Linux distributions.
- **Windows**: When using the Windows binary, you may need to set environment variables through the System Properties or use the `set` command in the command prompt before running the application.
- **macOS**: The macOS binary should work similarly to the Linux version. Ensure you've built it on a macOS system as described in the "Building for macOS" section.
