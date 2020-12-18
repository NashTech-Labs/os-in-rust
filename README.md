# Minimal Rust Kernel
This repository pertains to creating a Minimal Rust Kernel and printing text to the screen as well using QEMU emulator. To create that kernel we need to follow certain steps:

## Steps to create a bare-metal executable:
* Recompiling core library
* Enabling compiler-builtins-mem feature
* Setting-up default target
* Creating a BootImage
* Execute Bootable Image

To understand the logic behind this template. Please have a look at the blog series *OS in Rust*:
* [An executable that runs on bare metal: Part-1](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-1/)
* [An executable that runs on bare metal: Part-2](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-2/)
* [Custom target to build kernel for a bare metal: Part-3](https://blog.knoldus.com/os-in-rust-custom-target-to-build-kernel-for-a-bare-metal-part-3/)
* [Building kernel for custom target: Part-4](https://blog.knoldus.com/os-in-rust-building-kernel-for-custom-target-part-4/)
* Part-5 Coming soon...
