# Mundane OS

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
