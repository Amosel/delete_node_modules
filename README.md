# Delete node_modules

This is a command-line application that simplifies the task of finding and deleting `node_modules` directories, which often consume significant disk space over time. Built with Rust and a text-based user interface (TUI), it provides an intuitive way to navigate through directories, view `node_modules` folder sizes, and delete them selectively or in bulk.

## Features

- **Interactive TUI**: A clean, responsive text-based user interface for easy navigation and operation.
- **Bulk and Selective Deletion**: Toggle between directories and select specific `node_modules` for deletion or use bulk actions to handle multiple directories.
- **Safety and Control**: Review and confirm before you delete, ensuring that you don't accidentally remove necessary files.

## Installation

Before Delete node_modules, ensure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system.

```bash
# Clone the repository
git clone https://github.com/amosel/delete_node_modules.git

# Change to the app directory
cd delete_node_modules

# Build the project
cargo build --release

# Run the application
./target/release/delete_node_modules
