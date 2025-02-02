bits 16       ; real mode
org 0x7c00    ; loads code at this address

mov si, 0     ; counter

print:
    mov ah, 0x0e
    mov al, [hello + si]
    int 0x10    ; video services
    add si, 1
    cmp byte [hello + si], 0  ; check if 0
    jne print

jmp $          ; forever loop

hello:
    db "Hello, boot", 0

times 510 - ($ - $$) db 0  ; fill to fit inside boot sector
dw 0xAA55                  ; magic for boot sector fin
