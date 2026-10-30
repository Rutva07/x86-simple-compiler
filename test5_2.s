.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $16, %rsp
  movq $0, %rax
  movq %rax, -8(%rbp)
  movq $1, %rax
  movq %rax, -16(%rbp)
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq %rsi, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb whilebody
  jmp whileend

whilebody:
  movq -16(%rbp), %rax
  pushq %rax
  movq -8(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  movq -8(%rbp), %rax
  pushq %rax
  movq $1, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp whilecond

whileend:
  movq %rdi, %rax
  pushq %rax
  movq $4, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  je ifthen
  jmp ifelse

ifthen:
  movq -16(%rbp), %rax
  pushq %rax
  movq $10, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, -16(%rbp)
  jmp ifend

ifelse:
  movq -16(%rbp), %rax
  pushq %rax
  movq $5, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, -16(%rbp)
  jmp ifend

ifend:
  movq -16(%rbp), %rax
  addq $16, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits