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
