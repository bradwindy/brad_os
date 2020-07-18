# BradOS
A very basic OS written in Rust. Used to learn about writing operating systems. 

## Setup
**Run commands in project dir unless specified** 

0. Install Rust on your system - https://www.rust-lang.org/tools/install
1. Clone repo - `git clone https://github.com/bradwindy/brad_os.git`
2. Open folder in an IDE with Rust support, VSCode with Rust extension is recommended.
3. Run `rustup component add rust-src` in a terminal to allow our build tools to access the rust source code.
4. Run `rustup override add nightly` then `rustup update nightly --force` to make this rust project use a nightly version of rust to access some experimental features we need.
5. Run `cargo install bootimage` to install the bootimage package, allowing us to create a bootimage for our OS.

## Building the OS
After completing the above. Running `cargo build` should compile the OS. 

## Creating a Boot Image 
Running `cargo bootimage` should create a boot image that can be written to a USB disk, or run in a VM.

## Build, Boot Image, and VM Run Combo
**QEMU must be installed on the host system to run this command**

The command `cargo run` should compile the OS, create a boot image and run it in QEMU.
