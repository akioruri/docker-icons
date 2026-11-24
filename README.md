# docker-icons

English | [中文文档](README.zh.md)

A collection of Docker service icons, plus a small tool for merging the transparency of one image into another.

## Features

- **Icon set**: PNG icons for popular self-hosted services in `icons/` (e.g. `gitea`, `plex`, `qbittorrent`, `syncthing`, `nginx-proxy-manager`).
- **Alpha merge tool** (`docker-icons`): takes the alpha (transparency) channel from one image and applies it to the RGB colors of another. Useful for giving a flat-colored icon the shape and transparency of another image.

## Project Structure

```text
.
├── icons/          # Icon PNG files
├── src/main.rs     # Alpha merge tool source
├── Cargo.toml
└── LICENSE
```

## Installation

You need the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```sh
git clone git@akioruri.site:ruri/docker-icons.git
cd docker-icons
cargo build --release
```

The binary is written to `target/release/docker-icons`. To install it into `~/.cargo/bin`:

```sh
cargo install --path .
```

## Usage

### Merge the alpha channel

```sh
docker-icons <MASK> <FILL> [-o <OUTPUT>]
```

| Argument       | Description                                                                                    |
| -------------- | ---------------------------------------------------------------------------------------------- |
| `MASK`         | Image providing the alpha channel (shape and transparency)                                     |
| `FILL`         | Image providing the RGB colors                                                                 |
| `-o, --output` | Output PNG path. Defaults to `<FILL file stem>_with_alpha.png` in the same directory as `FILL` |

Examples:

```sh
# Writes logo_with_alpha.png
docker-icons mask.png logo.png

# Custom output path
docker-icons mask.png logo.png -o result.png
```

Notes:

- Both images should have the same dimensions. The tool does not check this; if they differ, the result may not be what you expect.
- The existing alpha of `FILL` is overwritten; only the alpha of `MASK` is kept.

## License

Released under the [MIT License](LICENSE). Copyright © 2026 ruri.
