ORG 0x7C00 ; load code at address
BITS 16 ; TODO get to 64

main:
    MOV ax, 0 ; constant starting point
    MOV ds, ax
    MOV es, ax
    MOV ss, ax

    MOV sp, 0x7c00
    MOV si, os_message
    CALL print
    HLT ; halt

halt:
    JMP halt

print:
    PUSH si ; pushing to stack
    PUSH ax
    PUSH bx

print_loop:
    LODSB
    OR al, al ; 0 is equal to 0
    JZ done_print

    MOV ah, 0x0E ; print a char
    MOV bh, 0
    INT 0x10 ; video interrupt

    JMP print_loop

done_print:
    POP bx
    POP ax
    POP si
    RET

os_message: DB 'Boot has in fact booted', 0x0D, 0x0A, 0 ; the bits at the end are new line chars, 0 is end of print

TIMES 510 - ($ - $$) DB 0
DW 0AA55h ; magic
