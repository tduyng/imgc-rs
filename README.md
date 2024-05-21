# Image Converter: imgc

Imgc is a Rust-based command-line application designed to convert various image formats using the [image](https://github.com/image-rs/image) crate. Currently, it only supports conversion to the WebP format.

It aims to provide a simple and efficient way to batch process image files for better performance and reduced storage space.

## Features

- Convert images from various formats to WebP using lossless encoding.
- Supports common image formats (PNG, JPEG, etc.).
- Customizable output directory for converted images.
- Parallel processing using [rayon](https://github.com/rayon-rs/rayon).

## Requirements

- Rust (latest stable version recommended)

## Installation

As the package is not yet published on crates.io, you need to clone the repository and use it locally for testing.

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
    
    If you want use this tool on your computer, you can install it from this repository:

    ```bash
    cargo install --path .
    ```
    After installation, you can use the command: `imgc`

## Usage

### Running the Program

**To convert images in a specific directory:**

```bash
cargo run --release webp -d examples
```

**To specify an output directory:**

```bash
cargo run --release webp -d examples -o output_images
```

**To clean generated files:**

```bash
cargo run --release clean -d examples -e webp
```

For a better understanding of each command, you can run `--help` or `-h` to display the help information.

```bash
cargo run -- -h
```

Output:

```bash
A CLI for converting images to the WebP format written in Rust

Usage: imgc <COMMAND>

Commands:
  webp   Convert images to webp format
  clean  Clean files by given extension
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```


To display help for the webp command:

```bash
cargo run webp -h
```

Output:

```bash
Convert images to webp format

Usage: imgc webp [OPTIONS] --dir <DIR>

Options:
  -d, --dir <DIR>        (Required) Directory containing images to process
  -o, --output <OUTPUT>  (Optional) Output directory for processed images. Defaults to the same location as the original images
  -h, --help             Print help

```

To display help for the clean command:
```bash
cargo run clean -h
```

```bash
Clean files by given extension

Usage: imgc clean --dir <DIR> --ext <EXT>

Options:
  -d, --dir <DIR>  (Required) Directory to clean files
  -e, --ext <EXT>  (Required) Extension of files to clean
  -h, --help       Print help
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

The command will convert all PNG, JPEG, and other supported images to WebP, placing them in the specified output directory or alongside the originals if no output directory is specified.

Example of webp command:

![Webp command example](/docs/img/webp_cmd.webp)

Example of clean command:

![Clean command example](/docs/img/clean_cmd.webp)

## Roadmap
- [ ] Testing
- [ ] Error handling enhancements
- [ ] Advanced options: single image file, exclude fgsolders, quality, resizing options...
- [ ] Progress reporting: show progress for large batch conversions.
- [ ] Integration with image metadata: preserve metadata during the conversion process.
- [ ] Parallel processing improvements: further optimize performance with advanced multithreading techniques.
- [ ] Supports conversion to various formats


## License

[MIT License](./LICENCE_MIT) or [Apache License](./LICENSE_APACHE)


