# Information
## Why this project exists
I made this to learn about game engine development and the process behind it. Perhaps in the future I could even make something using this engine.

## Technical information
The engine utilises Vulkan an Rust.

## Builds
The engine is currently only built for Linux and only tested on Arch. But the goal is to have a multiplatform engine.

# Setup
## Linux
To build the engine on linux you require a set of dependencies.
- Rust packages
- Vulkan packages
- Packages for whatever gpu you are using, sice I only know the packages for Nvidia I will not cover these.

#### Arch
On arch you need to run the following command to install the needed packages.
```
sudo pacman -S rustup vulkan-tools vulkan-devel
```

Then you need to install rust. As of now the latest default stable release works.
```
rustup default stable
```

- Note:
If building fails cause shaderc-sys has a mental breakdown for absolutely no reason, you might need to install a Cmake version <4 and then run a build with an extra flag:
```
export CXXFLAGS="-include cstdint"
cargo clean && cargo build
```

If this doesnt fix it. God be with you.