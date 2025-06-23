/*!
# Image Converter (imgc)

Welcome to imgc!

This tool is a command-line utility built with Rust,
 focusing on converting images into the WebP format.
Leveraging the powerful [image crate](https://github.com/image-rs/image),
 imgc simplifies the process of batch converting images,
 optimizing for both performance and storage efficiency.

## Key Features

- **Efficient Conversion**: 
 Convert a variety of image formats to WebP or AVIF with lossy or lossless compression.
- **Broad Format Support**: 
 Works with many popular image formats, including PNG and JPEG.
- **Custom Output**:
 Choose where your converted images are saved.
- **Speedy Processing**:
 Takes advantage of `rayon` for fast, parallel processing.
- **Glob Pattern Support**: 
 Since imgc is not yet available on crates.io, you'll need to clone the repository to get started:

## Getting Started

### Prerequisites

- Ensure you have the latest stable version of `Rust` and `Cargo` installed on your system.

### Installation Guide

Since `imgc` is not yet available on crates.io,
 you'll need to clone the repository to get started:

1. Clone the repository:

    ```bash
    git clone https://github.com/Gunzinger/imgc-rs.git
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
4. Install from GitHub
    
    If you want to test this tool without cloning the repository,
     you can install it directly from git:

    ```bash
    cargo install --git https://github.com/Gunzinger/imgc-rs.git
    ```
   Once installed, you can start using the `imgc` command.

5. Uninstall

    ```bash
    cargo uninstall imgc
    ```

## How to Use imgc

### Basic Usage

The `imgc` program uses glob patterns for target selection:

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

**Warning**: Use this command with caution. This is basically `rm -rf` with regex.

```bash
imgc clean "examples/**/*.webp"
```

### Command Help

For detailed command usage, see all arguments with `--help` or `-h`:

```bash
❯ imgc -h              
A configurable and efficient batch image converterwritten in Rust.

Usage: imgc <COMMAND>

Commands:
  webp   Convert images to webp format
  avif   Convert images to avif format
  clean  Remove files matching a glob pattern
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
  <PATTERN>  Glob pattern to match images to convert. Example: `images/**/*.png`

Options:
  -o, --output <OUTPUT>     (Optional) Output of processed images. Defaults to the same location as the original images
      --overwrite-existing  (Optional) Overwrite existing outputs? Defaults to false. (Determined by filename match)
      --lossless            (Optional) Use lossless encoding mode. Defaults to false
  -q, --quality <QUALITY>   (Optional) Control target quality for encoding (0 - 100, lower is worse). Defaults to 90.0
  -h, --help                Print help
```

For the `avif` command:

```bash
❯ imgc avif -h                                     
Convert images to avif format

Usage: imgc avif [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  Glob pattern to match images to convert. Example: `images/**/*.png`

Options:
  -o, --output <OUTPUT>     (Optional) Output of processed images. Defaults to the same location as the original images
      --overwrite-existing  (Optional) Overwrite existing outputs? Defaults to false. (Determined by filename match)
  -q, --quality <QUALITY>   (Optional) Control target quality for encoding (0 - 100, lower is worse). Defaults to 90.0
  -s, --speed <SPEED>       (Optional) Control encoding speed (1 - 10, lower is much slower but has a better quality and lower filesize). Defaults to 3
  -h, --help                Print help
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

Using `imgc`, you can convert all [supported images](https://docs.rs/image/0.25.6/image/codecs/index.html#supported-formats) to WebP or AVIF, saving them either in a specified directory or alongside the original files.

Example of webp command:

![Webp command example](/docs/img/webp_cmd.webp)

Example of clean command:

![Clean command example](/docs/img/clean_cmd.webp)

## What's Next
- [ ] Testing
- [ ] Introduce advanced options for resizing
- [ ] Expand support for additional conversion formats and encoding libraries

## License

This project under the [MIT License](LICENCE)
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
