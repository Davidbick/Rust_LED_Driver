# Embedded Rust Animations

This project implements an accelerometer-driven LED animation system using **Rust** on the **Adafruit Feather RP2040**.

### Design Notes

- Drivers use **embedded-hal traits** → reusable across MCUs
- Animations implement a shared `Animation` trait:
  - `next()` → advance frame
  - `to_list()` → generate 8×8 output
- `main.rs` handles:
  - hardware setup (HAL)
  - sensor reading
  - animation selection

## Build & Flash Instructions

### 1. Install Dependencies

- Rust toolchain (with `thumbv6m-none-eabi`)
- `elf2uf2-rs`
- Feather RP2040 board support

### 2. Build and Flash

1. Hold **BOOTSEL**
2. Plug in or reset the board
3. Run:

```
cargo run
```