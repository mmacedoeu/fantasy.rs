# Setup instructions

## Install

[![Language (Rust)](https://img.shields.io/badge/powered_by-Rust-blue.svg)](http://www.rust-lang.org/)

To compile and install you need to first install Rust [compiler](https://www.rust-lang.org/en-US/install.html)

`curl https://sh.rustup.rs -sSf | sh`

### Compile for release

`cargo build --release`

[![asciicast](https://asciinema.org/a/k9DN3Y5RrraPkLw5ZvYlvR1JO.png)](https://asciinema.org/a/k9DN3Y5RrraPkLw5ZvYlvR1JO)

### Run all tests

`cargo test --all`

[![asciicast](https://asciinema.org/a/XTUjML8d9YTnOLEB9G4vZAm8X.png)](https://asciinema.org/a/XTUjML8d9YTnOLEB9G4vZAm8X)

## Usage

Display help:
`./target/release/fantasy --help`

```text
fantasy 0.1.0
mmacedoeu <contato@mmacedo.eu.org>
Breath of Fantasy

USAGE:
    fantasy [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <base>           Specify the base storage path.
    -f <config>         File name with configuration parameters
    -p <players>        Number of players on battle
```

[![asciicast](https://asciinema.org/a/fIQcqrnnFa3y9bAi2tZqG5GhN.png)](https://asciinema.org/a/fIQcqrnnFa3y9bAi2tZqG5GhN)

[![asciicast](https://asciinema.org/a/TP8v5rRXgg0FRDqukUoM24AfX.png)](https://asciinema.org/a/TP8v5rRXgg0FRDqukUoM24AfX)

## Platform support

Should compile and work on all rust compiler supported [plataforms](https://forge.rust-lang.org/platform-support.html) but only tested for 64bit linux

### Docker support

There is a docker image on docker hub

Fetch
`docker pull mmacedoeu/fantasy`
Or build from source
`docker build -f docker/alpine_edge/Dockerfile --no-cache -t mmacedoeu/fantasy .`

And run
`docker run -it mmacedoeu/fantasy`


### Snapd support

Look for Snap install [instructions](https://docs.snapcraft.io/core/install) for your OS

`sudo snap install mmacedo-fantasy --channel=edge --devmode`

[![asciicast](https://asciinema.org/a/kC45InKCGpHF1vQDVNgBAHOSP.png)](https://asciinema.org/a/kC45InKCGpHF1vQDVNgBAHOSP)

### Continous integration

Travis support is provided with file `.travis.yml` but you need
to setup web hooks yourself on project

## Documentation

Follow instructions on https://github.com/rust-lang-nursery/mdBook to build documentation inside docs

## Api Docs

For deep instructions see: https://doc.rust-lang.org/beta/rustdoc/what-is-rustdoc.html#using-rustdoc-with-cargo

`cargo doc`

## Profiling and performance

See [here](./chapter_3.md)