
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 9
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_8_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00hello_world.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00power_3.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00second.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00write_a.bin"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01power_5.bin"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01write_b.bin"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02power_7.bin"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
app_7_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02write_c.bin"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
app_8_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/03sleep.bin"
app_8_end:
