global long_mode_start

section .text
bits 64
long_mode_start:
  extern rust_main
  call rust_main 

  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8000], rax
  hlt
