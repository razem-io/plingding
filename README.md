# PlingDing

This is a Rust application that sends push notifications via pushover.net. It can be used from the command line to send messages with optional priority and image attachments.

## Prerequisites

1. Rust and Cargo installed on your system
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

## Building

To build the application, run:

```
cargo build --release
```

The compiled binary will be available in `target/release/plingding`.

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
   ./plingding -m "Hello, World!"
   ```

2. Send a message with high priority:
   ```
   ./plingding -m "Urgent message" -p 2
   ```

3. Send a message with an attached image:
   ```
   ./plingding -m "Check out this image" -i /path/to/image.jpg
   ```

## Note

Make sure to set the PUSHOVER_API_TOKEN and PUSHOVER_USER_KEY environment variables before running the application. If these environment variables are not set, the application will return an error message indicating which variable is missing.
