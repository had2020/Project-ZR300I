    .section .text
    .global efi_main
    .type efi_main, %function

efi_main:
    // Save registers if needed
    // Load the EFI System Table pointer (2nd argument in x1)
    MOV X2, X1       // Save the System Table pointer for later

    // Load the address of the "Hello, World!" string
    ADR X0, hello_string  // ADR loads the relative address into X0

    // Call Print function (requires calling conventions and Print pointer)
    BL printf

    // Exit with EFI_SUCCESS
    MOV X0, #0        // EFI_SUCCESS = 0
    RET               // Return to UEFI loader

    .section .data
hello_string:
    .asciz "Hello, UEFI World!\n"
