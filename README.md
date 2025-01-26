# Rusivoran Os

you must build with -xbuild, due to no_std, requirment. UNFIN
'''
 cargo install cargo-xbuild
'''

WIP

arm64 bootloader
'''
cargo build --target aarch64-unknown-uefi
'''

cargo build --release --target aarch64-unknown-uefi -Z build-std


build the iso for Arm64
'''
sh BuildARM.sh
'''

qemu TEST

'''
qemu-system-aarch64 -machine virt -cpu cortex-a57 -m 1024 \
  -bios QEMU_EFI.fd -drive file=output.iso,format=raw
'''

testing tools
'''
objdump -x Bootloader_architectures/arm64/target/aarch64-unknown-uefi/release/arm64.efi
'''

for arm64 rustup override set nightly
rustup component add rust-src


arm toolchain for bootloader

brew install gcc-arm-embedded

linux
sudo apt update
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi

rustup override set stable
rustup override set nightly
