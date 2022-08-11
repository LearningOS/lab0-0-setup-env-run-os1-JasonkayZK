# build target
cargo build

# show file format
file target/x86_64-unknown-none/debug/os-gnu
# target/x86_64-unknown-none/debug/os-gnu: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, not stripped

# show header
rust-readobj -h target/x86_64-unknown-none/debug/os-gnu

#File: target/x86_64-unknown-none/debug/os-gnu
#Format: elf64-x86-64
#Arch: x86_64
#AddressSize: 64bit
#LoadName: <Not found>
#ElfHeader {
#  Ident {
#    Magic: (7F 45 4C 46)
#    Class: 64-bit (0x2)
#    DataEncoding: LittleEndian (0x1)
#    FileVersion: 1
#    OS/ABI: SystemV (0x0)
#    ABIVersion: 0
#    Unused: (00 00 00 00 00 00 00)
#  }
#  Type: SharedObject (0x3)
#  Machine: EM_X86_64 (0x3E)
#  Version: 1
#  Entry: 0x1210
#  ProgramHeaderOffset: 0x40
#  SectionHeaderOffset: 0x1630
#  Flags [ (0x0)
#  ]
#  HeaderSize: 64
#  ProgramHeaderEntrySize: 56
#  ProgramHeaderCount: 7
#  SectionHeaderEntrySize: 64
#  SectionHeaderCount: 19
#  StringTableSectionIndex: 17
#}

# Inverse Assemble
rust-objdump -S target/x86_64-unknown-none/debug/os-gnu

#target/x86_64-unknown-none/debug/os-gnu:        file format elf64-x86-64
#
#Disassembly of section .text:
#
#0000000000001210 <_start>:
#; }
#    1210: 31 c0                         xorl    %eax, %eax
#    1212: c3                            retq
