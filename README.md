# rust-chip8
## [Chip8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust
Small practice project in new (to me) language.
I used [Cowgod's Chip-8 technical reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) as a main ref sheet.
## Build
You'll need SDL2 and stable Rust installed. Then just run 
```
cargo build
```
and it should run fine (If it doesn't, please open an issue).
## Usage
Open the executable with a path to the ROM. One working program (MAZE) is supplied.
I do it using cargo:
```
cargo run MAZE
```
