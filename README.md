# Image Converter `imgc`

`imgc` is a command-line utility focusing on converting images into other formats,
 specifically targeting support modern image standards and encoders.
Leveraging the powerful [image crate](https://github.com/image-rs/image),

`imgc` simplifies the process of batch converting images,
 optimizing for both performance and storage efficiency.

### Usage example using the docker container:
```bash
> docker run -v ./examples/:/targets/ -it gunzinger/imgc-rs:latest imgc avif "**/*.*"
Converting 16 files...
Using "ravif" (0.12.0) with options (quality: 90, speed: 3, bit depth: Eight, color model: RGB)
Encode statistics:
Successful: 15
Skipped:    0
Errors:     0
Total input size:  24.0 MiB
Total output size: 13.2 MiB
Compression ratio: 54.95%
```

---

## Key Features

- **Efficient Conversion**: 
 Convert a variety of image formats to WebP or AVIF with lossy or lossless compression.
- **Broad Format Support**: 
 Works with many [supported image formats](https://docs.rs/image/0.25.6/image/codecs/index.html#supported-formats).
- **Custom Output**:
 Choose where your converted images are saved.
- **Speedy Processing**:
 Takes advantage of `rayon` for fast, parallel processing.
- **Glob Pattern Support**: 
 Target selection is made intuitive for cli enthusiasts via glob patterns.

---

## Installation

### Using published binaries

Binaries for Windows and Linux are built for every tag.

See the [GitHub releases](https://github.com/Gunzinger/imgc-rs/releases) page for downloads.

### Using the docker image

Docker containers are also built for every tag.

See the [Docker Hub](https://hub.docker.com/r/gunzinger/imgc-rs) page for available tags.

```bash
docker run -it gunzinger/imgc-rs:latest imgc --help

# directory passthrough on linux
docker run -v ./input-folder/:/targets/ -it gunzinger/imgc-rs:latest imgc avif "/targets/**/*.png"

# note that on windows the volume passthroughs need to have absolute paths, e.g. (for powershell)
docker run -v ${PWD}/input-folder/:/targets/ -it gunzinger/imgc-rs:latest imgc avif "/targets/**/*.png"

```

---

## How to Use `imgc`

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

---

### Command Help

For detailed command usage, see all arguments with `--help` or `-h`:

```bash
❯ imgc -h              
A configurable and efficient batch image converter written in Rust.

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
> imgc clean -h                  
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

Using `imgc` you can convert all [supported images](https://docs.rs/image/0.25.6/image/codecs/index.html#supported-formats)
 to WebP or AVIF, saving them either in a specified directory or alongside the original files.

Example of webp command:

![Webp command example](/docs/img/webp_cmd.webp)

Example of clean command:

![Clean command example](/docs/img/clean_cmd.webp)

---

## Building from source

### Prerequisites

- Ensure you have the latest stable version of `Rust` and `Cargo` installed on your system.
- [Nasm](https://www.nasm.us/) is needed for building `rav1e`.
  Install via `apt install nasm` / `apk add nasm` / `choco install nasm`.

### Installation Guide

#### Install via crate

To install via the [published crate](https://crates.io/crates/imgc), execute the following command:

```bash
cargo install imgc
```

#### Install from git

```bash
# 1. Clone the repository:
git clone https://github.com/Gunzinger/imgc-rs.git
cd imgc-rs
# 2. Build the project:
cargo build --release
3. Install locally
cargo install --path .
```

#### Uninstalling

To uninstall, remove the tool via `cargo uninstall`:

```bash
cargo uninstall imgc
```

---

## What's Next

- [ ] Testing
- [ ] Introduce advanced options for resizing
- [ ] Expand support for additional conversion formats and encoding libraries

---

## License

This project under the [MIT License](LICENCE)
