# Papyrus Wallpaper

A lightweight, fast, and easy-to-use application to set wallpapers individually per monitor or stretched across multiple monitors on Windows.

## 🚀 Features

- 🖼️ Set wallpapers per monitor or stretched across all screens
- 🪟 Windows support
- 🔧 Planned configuration options

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (only required if building from source)
- Windows OS (required)

### 🛠️ Option 1: Download Release (.exe)

Download the latest ready-to-use `.exe` file from the [Releases page](https://github.com/wurstcasesenario/papyrus-wallpaper/releases), and run it directly — no setup required.

---

### 🧑‍💻 Option 2: Build From Source

If you want to compile the project yourself:

To install Rust (if you haven't already), run this command in PowerShell or your terminal or [download it](https://www.rust-lang.org/tools/install):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then run the following commands to clone and build the project:

```bash
git clone https://github.com/wurstcasesenario/papyrus-wallpaper.git
cd papyrus-wallpaper
cargo build --release
```

You can now find the executable in the target/release folder.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
