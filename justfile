alias bw := build-windows
alias bm := build-mac
alias bl := build-linux
alias ba := build-all
alias pw := package-windows
alias pm := package-mac
alias pl := package-linux
alias pa := package-all

_default:
    @just --list --justfile {{ justfile() }}

# Test using cargo nextest
test *args:
    cargo nextest run {{ args }}

# Clean cargo artifacts and dist directory
@clean:
    cargo clean
    rm -rf dist

windows := "x86_64-pc-windows-gnu"
mac_x86 := "x86_64-apple-darwin"
mac_arm := "aarch64-apple-darwin"
linux := "x86_64-unknown-linux-musl"

# Build .exe for Windows
build-windows *args:
    cargo zigbuild -r --target {{ windows }} {{ args }}
    upx ./target/{{ windows }}/release/tasker-cli.exe

# Build executables for x86 and ARM Mac's
build-mac *args:
    cargo zigbuild -r --target {{ mac_x86 }} {{ args }}
    cargo zigbuild -r --target {{ mac_arm }} {{ args }}

# Build statically linked Linux executable
build-linux *args:
    cargo zigbuild -r --target {{ linux }} {{ args }}
    upx ./target/{{ linux }}/release/tasker-cli

# Build release executables for all three major operating systems
build-all: build-windows build-mac build-linux

# Package Linux executable
package-linux:
    @mkdir -p dist
    ouch c --slow ./target/{{ linux }}/release/tasker-cli README.md COPYING ./dist/tasker-cli-linux.tar.gz

# Package Windows executable
package-windows:
    @mkdir -p dist
    ouch c --slow ./target/{{ windows }}/release/tasker-cli.exe README.md COPYING ./dist/tasker-cli-windows.zip

# Package Mac executables
package-mac:
    @mkdir -p dist
    ouch c --slow ./target/{{ mac_x86 }}/release/tasker-cli README.md COPYING ./dist/tasker-cli-mac-x86.zip
    ouch c --slow ./target/{{ mac_arm }}/release/tasker-cli README.md COPYING ./dist/tasker-cli-mac-arm.zip

# Package all executables
package-all: build-all package-windows package-mac package-linux
