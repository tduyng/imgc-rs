# Image Converter `imgc` 🗜️

`imgc` is a command-line utility focusing on converting images into other formats,
 specifically focusing on support for modern image standards and encoders.

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

## Key Features 🧰

- **Broad Format Support**: 
 Works with many [supported image formats](#supported-formats).
- **Speedy Processing**:
  Written in Rust to keep overhead to a minimum, we also take advantage of `rayon` for parallel processing.
- **Input selection using Glob Patterns**:
  Target selection is made intuitive for cli enthusiasts via glob patterns.
- **Custom Output**:
 Choose where your converted images are saved.

---

## Supported formats

### Input formats 🖼️

To keep it simple: `JPEG`, `PNG`, `GIF`, `WebP`, `BMP`, `DDS`, `Farbfeld`, `HDR`, `ICO`, `EXR`, `PNM`, `QOI`, `TGA`, `TIFF`

Input images are decoded using the `image` crate,
 please see [their documentation for supported image formats](https://docs.rs/image/0.25.6/image/codecs/index.html#supported-formats).

### Output formats 📤

- `webp`, webp encoder using the `webp` crate (libwebp bindings) - offers lossy and lossless encoding
- `webp-image`, webp encoder using the `image` crate - offers lossless encoding
- `avif`, avif encoder using the `ravif` crate - offers lossy and lossless encoding
- `png`, png encoder using the `image` crate - offers lossless encoding
- `jpeg`, jpeg optimizer using the `mozjpeg` crate - only optimizes images

### Requests

If this does not cover your needs,
 please feel free to open an issue to request additional input and/or output formats.

I am focusing on supporting modern image formats supported in browsers,
 as this tool is optimally suited for optimizing static directories for different web apps.

For a good overview of browser support, see the [caniuse.com](https://caniuse.com) pages
 for different images, e.g.: [avif](https://caniuse.com/avif), [webp](https://caniuse.com/webp).

---

## Installation 💾

### Using published binaries 📡

Binaries for Windows and Linux are built for every tag.

See the [GitHub releases](https://github.com/Gunzinger/imgc-rs/releases) page for downloads.

### Using the docker image 🐳

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

## How to Use `imgc` 🧑‍💻

### Basic Usage

The `imgc` program uses glob patterns for target selection:

```bash
imgc webp "examples/**/*.png"
imgc webp "examples/**/*.jpg"
imgc webp "examples/**/*"
```

### Specifying an output directory 🗃️

```bash
imgc webp "examples/**/*" -o output_images
```

### Cleaning up generated files 🧹

**Warning**: Use this command with caution. This is basically `rm -rf` with regex.

```bash
imgc clean "examples/**/*.webp"
```

---

### Command Help 📖

For detailed command usage, see all arguments with `--help` or `-h`:

```bash
❯ imgc --help
A configurable and efficient batch image converter written in Rust.

Usage: imgc [OPTIONS] <PATTERN> <COMMAND>

Commands:
  webp        Convert images to webp format (using webp crate)
  webp-image  Convert images to webp format (using image crate)
  avif        Convert images to avif format (using ravif crate)
  png         Convert images to png format (using image crate)
  jpeg        Convert images to optimized jpeg format (using mozjpeg crate)
  clean       Remove files matching a glob pattern
  help        Print this message or the help of the given subcommand(s)

Arguments:
  <PATTERN>  Glob pattern to match images to convert. Example: `images/**/*.png`

Options:
  -o, --output <OUTPUT>               Output directory (flat) of processed images. Defaults to the same location as the original images with the new file extension
      --overwrite-if-smaller          Overwrite the existing output file if the current conversion resulted in a smaller file
      --overwrite-existing            Overwrite existing output files regardless of size
      --discard-if-larger-than-input  Discards the encoding result if it is larger than the input file (does not create an output file)
  -h, --help                          Print help
  -V, --version                       Print version
```

For the `webp` command:

```bash
❯ imgc webp --help                                     
Convert images to webp format (using webp crate)

Usage: imgc <PATTERN> webp [OPTIONS]

Options:
      --lossless           Use lossless encoding mode. Defaults to false
  -q, --quality <QUALITY>  Control target quality (0 - 100, lower is worse but results in smaller files). Defaults to 90.0
  -h, --help               Print help
```

For the `webp-image` command:

```bash
❯ imgc webp-image --help                                     
Convert images to webp format (using image crate)

Usage: imgc <PATTERN> webp-image

Options:
-h, --help  Print help
```

For the `avif` command:

```bash
❯ imgc avif --help                                     
Convert images to avif format (using ravif crate)

Usage: imgc <PATTERN> avif [OPTIONS]

Options:
  -q, --quality <QUALITY>
          Control target quality (0 - 100, lower is worse but results in smaller files). Defaults to 90.0
  -s, --speed <SPEED>
          Control encoding speed (1 - 10, lower is much slower but has a better quality and lower filesize). Defaults to 3
      --bit-depth <BIT_DEPTH>
          Choose internal bit depth. (in the generated avif file, nothing to do with the input file) [possible values: eight, ten, auto]
      --color-model <COLOR_MODEL>
          Choose internal color model. (in the generated avif file, nothing to do with the input file) [possible values: y-cb-cr, rgb]
      --alpha-color-mode <ALPHA_COLOR_MODE>
          Choose internal alpha color mode. (in the generated avif file, nothing to do with the input file) Irrelevant for images without transparency [possible values: unassociated-dirty, unassociated-clean, premultiplied]
  -a, --alpha-quality <ALPHA_QUALITY>
          Control target alpha quality (0 - 100, lower is worse). Defaults to 90.0
  -h, --help
          Print help
```

For the `png` command:

```bash
❯ imgc png --help                                     
Convert images to png format (using image crate)

Usage: imgc <PATTERN> png

Options:
      --compression-type <COMPRESSION_TYPE>
          Choose the png compression type
          See: https://docs.rs/image/latest/image/codecs/png/enum.CompressionType.html

          [possible values: default, fast, best]
      --filter-type <FILTER_TYPE>
          Choose the png filter type

          See: https://docs.rs/image/latest/image/codecs/png/enum.CompressionType.html

          [possible values: no-filter, sub, up, avg, paeth, adaptive]

  -h, --help
          Print help (see a summary with '-h')
```

For the `jpeg` command (unstable; likes to crash! this is a work in progress!):

```bash
❯ imgc jpeg --help                                     
Convert images to webp format (using mozjpeg crate)

Usage: imgc <PATTERN> jpeg

Options:
-h, --help  Print help
```

For the `clean` command:

```bash
> imgc clean --help                  
Remove files matching a glob pattern

Usage: imgc clean <PATTERN>

Arguments:
  <PATTERN>  Glob pattern to match files to remove

Options:
  -h, --help  Print help
```

---

## Examples

### Input Directory Structure

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
...
```

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
- [x] Publishing automation (binaries, docker)
- [ ] Introduce advanced options for image transformations (resize, rotate)
- [x] Progress bar for encoding
- [ ] Expand support for additional input formats 
  - [x] `avif`
  - [x] `png`
  - [ ] `jpeg` (WIP)
  - [ ] `png` (via `oxipng` crate)
  - [ ] `heic/heif`
  - [ ] `jxl/jpeg-xl`
  - [ ] incoming wishes
- [ ] Expand support for additional export formats by including more encoding libraries
- [ ] Image metadata handling (EXIF data preservation/stripping)
- [ ] Expand support for animated images/video encoding (to webp/avif/apng)
- [ ] Output logs (to enable usage in automations static directory optimizations by link-rewriting)
- [ ] `winresource` integration (application icon and .exe metadata for Windows binaries)
- [ ] GUI

---

## License

This project under the [MIT License](LICENCE).
