# Starter template for creating PS Vita applications using rust

Install cargo make with

`cargo install cargo-make`

Then compile the with

`cargo make`

And copy the resulting `vpk` from `target/arm7-vita-eabihf/release` to your Vita.
This step can be partly automated by starting an ftp server from your Vita and running `./copy-target.sh <ftp-server-address>`

Most credit for this code goes to the [Vita Rust](https://github.com/vita-rust) project
