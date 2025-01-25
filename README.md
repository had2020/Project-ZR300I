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
