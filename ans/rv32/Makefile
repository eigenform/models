
PREFIX 	:= riscv32-unknown-elf
CC 		:= $(PREFIX)-gcc
CFLAGS 	:= -march=rv32i -mabi=ilp32 -nostdlib
OBJDUMP := $(PREFIX)-objdump
OBJCOPY := $(PREFIX)-objcopy

all:
	$(CC) $(CFLAGS) test.c -o test.elf
	$(OBJCOPY) -O binary -j .text test.elf test.bin
disas:
	$(OBJDUMP) -M numeric --visualize-jumps -d test.elf
objdump:
	$(OBJDUMP) -x test.elf
clean:
	rm -fv test.elf test.bin
