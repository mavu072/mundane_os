# Mundane OS
A Rust OS kernel based on [os.phil-opp.com](https://os.phil-opp.com/).

### Bare Metal Environment
We need a bare metal environment with no operating system to compile our executable.

1. Add bare metal environment in rustup:
```
rustup target add thumbv7em-none-eabihf
```

Compile our executable for a bare metal target:
```
cargo build --target thumbv7em-none-eabihf
```

#### Compiling for the host system
We can compile for the host system by passing additional linker arguments:
```
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

#### Compiling for the custom target specification
We can compile our kernel with a custom target using our own target specification.
```
 cargo build --target x86_64-mundane_os.json
```
`x86_64-mundane_os.json` is set as the default target in `.cargo/config.toml` so we can build the kernel using a simple `cargo build`.

### Creating a Boot image
To run our kernel, we need to turn it into a bootable disk image. For this, we need to link the compiled kernel with a bootloader.
We use the `bootloader` crate which implements a basic BIOS bootloader without any C dependencies, only Rust and inline assembly.

To link the bootloader to the kernel we use `bootimage`, it compiles the kernel and bootloader, then links them together to create a bootable disk image.
Install the tool, from your home directory or any other except the cargo project:
```
cargo install bootimage
```

For running `bootimage`, you'll need the rustup component:
```
rustup component add llvm-tools-preview
```

#### Creating a bootable disk image
After installing `bootimage` and `llvm-tools-preview` component, you can create a bootable disk image from the cargo project directory:
```
cargo bootimage
```
This creates a bootable disk image in `target/x86_64-mundane_os/debug`. You can boot it in a virtual machine or copy to a USB to boot into physical hardware.
