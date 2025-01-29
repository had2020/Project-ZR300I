.section .text
.global _start

// UEFI Entry Point
_start:
    // Return to UEFI firmware (Exit Boot Services would be here in a real loader)
    mov x0, #0         // EFI_SUCCESS (0)
    mov x8, #0x5F      // UEFI ExitBootServices syscall number
    svc #0             // Make system call
    b .               // Loop forever if it returns (should never happen)
