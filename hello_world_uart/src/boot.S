.section .text.boot

.global _start
_start:
    mrs x0, mpidr_el1
    and x0, x0, #0xFF
    cbz x0, master
    b proc_hang

proc_hang:
    b proc_hang

master:
    mov sp, 0x80000
    bl main
    b proc_hang