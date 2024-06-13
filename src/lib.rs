/*!
# Image Converter (imgc)

Welcome to imgc!

This tool is a command-line utility built with Rust, focusing on converting images into the WebP format. Leveraging the powerful [image crate](https://github.com/image-rs/image), imgc simplifies the process of batch converting images, optimizing for both performance and storage efficiency.

## Key Features

- **Efficient Conversion**: Easily convert a variety of image formats to WebP with lossless compression.
- **Broad Format Support**: Works with many popular image formats, including PNG and JPEG.
- **Custom Output**: Choose where your converted images are saved.
- **Speedy Processing**: Takes advantage of `rayon` for fast, parallel processing.
- **Glob Pattern Support**: Since imgc is not yet available on crates.io, you'll need to clone the repository to get started:


## Getting Started

### Prerequisites

- Ensure you have the latest stable version of `Rust` and `Cargo` installed on your system.

### Installation Guide

Since imgc is not yet available on crates.io, you'll need to clone the repository to get started:

1. Clone the repository:

    ```bash
    git clone https://github.com/tduyng/imgc-rs.git
    cd imgc-rs
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```
3. Install locally

    ```bash
    cargo install --path .
    ```
4. Install from Github

    If you want to test this tool without cloning the repository, you can install it directly from git:

    ```bash
    cargo install --git https://github.com/tduyng/imgc-rs.git
    ```
Once installed, you can start using imgc with the command `imgc`.

5. Uninstall

    ```bash
    cargo uninstall imgc
    ```

## How to Use imgc

### Basic Usage

Imgc program use glob patterns for easy file handling:

```bash
imgc webp "examples/**/*.png"
imgc webp "examples/**/*.jpg"
imgc webp "examples/**/*"
```

### Specifying an output directory

```bash
imgc webp "examples/**/*" -o output_images
```

### Cleaning up generated files
**Warning**: Use this command with caution.

```bash
imgc clean "examples/**/*.webp"
```

### Command Help

For detailed command usage, `--help` or `-h` will guide you through:

```bash
❯ imgc -h              
A CLI for converting images to the WebP format writtent in Rust

Usage: imgc <COMMAND>

Commands:
  webp   Convert images to webp format
  clean  Clean files by given extension
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For the `webp` command:

```bash
❯ imgc webp -h                                     
Convert images to webp format

Usage: imgc webp [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  Glob pattern to match images to convert. Example: `images/**/*.jpg`

Options:
  -o, --output <OUTPUT>  (Optional) Output of processed images. Defaults to the same location as the original images
  -h, --help             Print help
```

For the `clean` command:

```bash
❯ imgc clean -h                  
Remove files matching a glob pattern

Usage: imgc clean <PATTERN>

Arguments:
  <PATTERN>  Glob pattern to match files to remove

Options:
  -h, --help  Print help
```

## Example Directory Structure
Given the following directory structure:

```bash
examples
├── 1.png
├── 1.webp
├── img1
│   ├── 2.png
│   ├── 2.webp
│   └── img11
│       ├── 3.jpg
│       └── 3.webp
├── img2
│   ├── 4.jpeg
│   └── 4.webp
```

Using imgc, you can convert all supported images to WebP, saving them either in a specified directory or alongside the original files.

Example of webp command:

![Webp command example](https://raw.githubusercontent.com/tduyng/imgc-rs/main/docs/img/webp_cmd.webp)

Example of clean command:

![Clean command example](https://raw.githubusercontent.com/tduyng/imgc-rs/main/docs/img/clean_cmd.webp)

## What's Next
- [ ] Testing
- [ ] Introduce advanced options for compression, quality, and resizing
- [ ] Expand support for additional conversion formats

## License

Choose between [MIT License](./LICENCE_MIT) or [Apache License](./LICENSE_APACHE) as per your preference.
*/

#![deny(missing_docs)]
/// Command-line interface functionality.
pub mod cli;
/// Image conversion functionality.
pub mod converter;
/// Error handling for the application.
mod error;
/// Image formats supported by the application.
pub mod format;

/// Utility functions and helpers.
pub mod utils;

pub use error::Error;
