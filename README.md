# Image Converter: imgconv
Imgconv is a Rust-based command-line application designed to convert various image formats into other format using the [image](https://github.com/image-rs/image) crate.
Now it only supports conversion to WebP format.

It aims to provide a simple and efficient way to batch process image files for better performance and reduced storage space.

## Features
- Convert images from various formats to WebP using lossless encoding.
- Supports common image formats (PNG, JPEG, etc.).
- Customizable output directory for converted images.
- Parallel processing using [rayon](https://github.com/rayon-rs/rayon)

## Requirements
Rust (latest stable version recommended)

## Installation
As the package is not yet published on crates.io, for using or testing, you need to clone the repository and use it locally.

1. Clone the repository:

```bash
git clone https://github.com/tduyng/imgconv.git
cd imgconv
```

2. Build the project:

```bash
cargo build --release
```

## Usage
### Command-Line arguments
```
--dir <DIR>: Specify the directory to process.
--output <OUTPUT_DIR>: Specify the output directory for converted images (defaults to the same directory as the input).
```

### Running the program
To convert images in a specific directory:

```bash
cargo run --release --bin cv -- --dir examples
````

To specify an output directory:

```bash
cargo run --release --bin cv --dir examples --output output_images
```

### Example directory structure
Given the following directory structure:

```
examples
├── 1.png
├── 1.webp
├── img1
│  ├── 2.png
│  ├── 2.webp
│  └── img11
│     ├── 3.jpg
│     └── 3.webp
├── img2
│  ├── 4.jpeg
│  └── 4.webp
```

The command will convert all png, jpeg, and other supported images to webp, placing them in the specified output directory or alongside the originals if no output directory is specified.

## Roadmap
- [ ] Testing
- [ ] Error handling enhancements
- [ ] Advanced options: single image file, exclude fgsolders, quality, resizing options...
- [ ] Progress reporting: show progress for large batch conversions.
- [ ] Integration with image metadata: preserve metadata during the conversion process.
- [ ] Parallel processing improvements: further optimize performance with advanced multithreading techniques.
- [ ] Supports conversion to various formats


## License

[MIT License](./LICENCE_MIT) and [Apache License](./LICENSE_APACHE)


