.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $24, %rsp
  movq $100, %rax
  movq %rax, -8(%rbp)
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq $2, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja whilebody
  jmp whileend

whilebody:
  movq -8(%rbp), %rax
  pushq %rax
  movq $10, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, -8(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq $1, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, %rdi
  jmp whilecond

whileend:
  movq %rsi, %rax
  pushq %rax
  movq %rdx, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb ifthen
  jmp ifelse

ifthen:
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, %rdi
  jmp ifend

ifelse:
  movq %rdi, %rax
  pushq %rax
  movq $5, %rax
  pushq %rax
  movq %rsi, %rax
  popq %rbx
  imulq %rbx, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, %rdi
  jmp ifend

ifend:
  movq %rdi, %rax
  pushq %rax
  movq %rdx, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, %rdi
  movq %rdi, %rax
  addq $24, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits