.section .text
.global _start
_start:
    // load the address of message
    mov x0, #1           // stdout
    ldr x1, =message
    ldr x2, =message_len
    mov x8, #64          // write syscall
    svc #0               // call UEFI service

loop:
    wfe                  // wait for event, low-power halt
    b loop               // infinite loop

.section .data
message:
    .ascii "UEFI Boot Successful!\n"
message_len = . - message
