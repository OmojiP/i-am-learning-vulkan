
[Vulkan Guide](https://vkguide.dev/) を参考にRustで作りながら勉強するリポジトリ

# 0

## Rustのインストール

### Visual Studioの MSVC C++ビルドツール

Visual Studioを導入済みなのでスキップ

### rustのインストール

https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/setup?tabs=winget

```
winget install Rustlang.Rustup
```

新しいターミナルで確認

```
cargo --version
cargo 1.98.0 (797e8a9bc 2026-08-05)
rustc --version
rustc 1.98.0 (88d9e12ae 2026-08-18)
```

### vscode環境の準備

1. vscodeをインストール
2. rust-analyzer 拡張機能をインストール

### Hello, worldする

```
cargo init
cargo run
Hello, world!
```

## Vulkanのインストール

LunarG Vulkan SDKをインストール

https://www.lunarg.com/products/vulkan-sdk/

vulkansdk-windows-X64-1.4.357.0.exe

**管理者として実行**

![alt text](img/0/image.png)

![alt text](img/0/image-1.png)

## ashのインストール

```
cargo add ash
```

```rust
use ash::vk;

fn main() {
    println!("Vulkan version: {:?}", vk::make_api_version(0, 1, 3, 0));
}
```

## ImGui

```rust
cargo add imgui
```

```rust
use imgui::{Context, dear_imgui_version};

fn main() {
    imgui_test();
}

fn imgui_test() {
    let mut imgui = Context::create();

    imgui.set_ini_filename(None);

    println!("Dear ImGui version: {}", dear_imgui_version());
}
```

## glam

https://docs.rs/glam/latest/glam/

```rust
cargo add glam
```

```rust
use imgui::{Context, dear_imgui_version};

fn main() {
    glam_test();
}

fn glam_test() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("glam is installed: {:?}", v);
}
```
