Note: the circuit schematics in documentation/pcb-circuit-diagram.pdf are outdated. Only DCNF0 is grounded. DCNF1 is pulled up to 3.3V, which means that FT4222H is in mode 3 / CNFMOD2 (4 data streams, no GPIO control).

# Requirements

-   https://rustup.rs
-   https://rust-lang.github.io/rust-bindgen/requirements.html

# Usage

```sh
touch gps.nmea
cargo run --release
```
