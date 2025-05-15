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

## To-Do
- Add better logging and improve readability in the vulkan_wrapper. Specifically the physical device creation method.
- Remove the unsafe flags (Most vulkan API call functions are unsafe and need to be handled in a very specific way to make them safe. For simplicity sake all Vulkan calls are unsafe for now)