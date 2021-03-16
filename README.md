# VGA Text Mode
This repository pertains to the incorporation of VGA text mode in our kernel. Here we’ll create an interface through which we can use its functionality to print something on a screen. 
To create that interface we need to follow certain steps:



## Steps to create a bare-metal executable:
* Create Rust module and define colors
* Provide color code
* Provide text buffer
* Create Writer
* Adding implementation to the Writer

To understand the logic behind this template. Please have a look at the blog series *OS in Rust*:
* [An executable that runs on bare metal: Part-1](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-1/)
* [An executable that runs on bare metal: Part-2](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-2/)
* [Custom target to build kernel for a bare metal: Part-3](https://blog.knoldus.com/os-in-rust-custom-target-to-build-kernel-for-a-bare-metal-part-3/)
* [Building kernel for custom target: Part-4](https://blog.knoldus.com/os-in-rust-building-kernel-for-custom-target-part-4/)
* [Running our custom kernel on an emulator: Part-5](https://blog.knoldus.com/os-in-rust-running-our-custom-kernel-on-an-emulator-part-5/)
* [Incorporate VGA buffer: Part-6](https://blog.knoldus.com/os-in-rust-incorporate-vga-buffer-part-6/)
* [Incorporate VGA buffer: Part-7](https://blog.knoldus.com/os-in-rust-incorporate-vga-buffer-part-7/)
* Part-8 Coming soon...
