ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x40000000, LENGTH = 100M
}

SECTIONS
{
    . = 0x40000000;

    .text : {
        *(.text._start)
        *(.text*)
    } > RAM

    .rodata : {
        *(.rodata*)
    } > RAM

    .data : {
        *(.data*)
    } > RAM

    .bss : {
        *(.bss*)
    } > RAM

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}
