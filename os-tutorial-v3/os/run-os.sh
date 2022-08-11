export LOG=TRACE

# build code
cargo build --release

# strip meta segment
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

# start kernel via qemu
qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios ../bootloader/rustsbi-qemu.bin \
   -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

##################### quit qemu #####################
#   Ctrl + A, X

##################### debug qemu #####################
# server
#qemu-system-riscv64 \
#  -machine virt \
#  -nographic \
#  -bios ../bootloader/rustsbi-qemu.bin \
#   -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
#   -s -S

# client
#riscv64-unknown-elf-gdb \
#    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
#    -ex 'set arch riscv:rv64' \
#    -ex 'target remote localhost:1234'


##################### gdb command #####################

# x/10i $pc：从当前 PC 值的位置开始，在内存中反汇编 10 条指令
# si：让 Qemu 每次向下执行一条指令，之后屏幕会打印出待执行的下一条指令的地址
# p/x $t0： 以 16 进制打印寄存器 t0 的值，注意当要打印寄存器的时候需要在寄存器的名字前面加上 $
# b *0x80200000：地址 0x80200000 处打一个断点
# c：（Continue 的缩写）让 Qemu 向下运行直到遇到一个断点
# p/d $x1：以十进制打印寄存器 x1 的值
