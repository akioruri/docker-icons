# docker-icons

[English](README.md) | 中文文档

一组 Docker 服务图标，以及一个用于合并图片透明通道的小工具。

## 功能

- **图标集**：`icons/` 目录下收录了常用自托管服务的 PNG 图标（如 `gitea`、`plex`、`qbittorrent`、`syncthing`、`nginx-proxy-manager` 等）。
- **alpha 合并工具**（`docker-icons`）：用一张图片的透明通道（alpha）替换另一张图片的 alpha，颜色（RGB）保持不变。适合把一张纯色图标套上另一张图的形状与透明度。

## 目录结构

```text
.
├── icons/          # 图标 PNG 文件
├── src/main.rs     # alpha 合并工具源码
├── Cargo.toml
└── LICENSE
```

## 安装

需要先安装 [Rust 工具链](https://www.rust-lang.org/tools/install)。

```sh
git clone git@akioruri.site:ruri/docker-icons.git
cd docker-icons
cargo build --release
```

编译产物位于 `target/release/docker-icons`。也可以直接安装到 `~/.cargo/bin`：

```sh
cargo install --path .
```

## 使用

### 合并 alpha 通道

```sh
docker-icons <MASK> <FILL> [-o <OUTPUT>]
```

| 参数           | 说明                                                                         |
| -------------- | ---------------------------------------------------------------------------- |
| `MASK`         | 提供 alpha 通道的图片（形状、透明度来源）                                    |
| `FILL`         | 提供 RGB 颜色的图片（颜色来源）                                              |
| `-o, --output` | 输出 PNG 路径，默认为 `<FILL 文件名>_with_alpha.png`，与 `FILL` 位于同一目录 |

示例：

```sh
# 输出 logo_with_alpha.png
docker-icons mask.png logo.png

# 指定输出路径
docker-icons mask.png logo.png -o result.png
```

注意：

- 两张图片的尺寸应当一致。程序目前不会检查尺寸，尺寸不一致时结果可能不符合预期。
- `FILL` 原有的 alpha 会被覆盖，只保留 `MASK` 的 alpha。

## 许可证

本项目基于 [MIT License](LICENSE) 发布，版权所有 © 2026 ruri。
