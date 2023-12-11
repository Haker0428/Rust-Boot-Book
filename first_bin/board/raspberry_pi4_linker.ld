OUTPUT_ARCH(aarch64)
ENTRY(_start)
BASE_ADDRESS = 0x80000;

SECTIONS
{
    . = BASE_ADDRESS;

    .text.boot : {
        *(.text.boot)
    }

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    . = ALIGN(8);
    bss_begin = .;
    .bss : { *.(.bss*)}
    bss_end = .;
}