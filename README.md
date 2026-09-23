# hxcemu

## description

hxcemu is a simple emulator for the Hack computer. The Hack computer is a
16-bit machine based on an extremely minimal architecture. You can run
programs written in the Hack assembly language.

## build

```bash
cargo build
# or
cargo build --release
```

## usage

```bash
cargo run <.hack file>
# or
./target/debug/hxcemu <.hack file>
# or
./target/release/hxcemu <.hack file>
```

## tests

### unit

```bash
cargo test
```

## sources

* The Elements of Computing Systems, Building a Modern Computer from First
  Principles, Noam Nisan and Shimon Schocken
* http://nand2tetris.org

