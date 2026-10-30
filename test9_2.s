.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $32, %rsp
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  movq %rsi, %rax
  pushq %rax
  movq %rdx, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  movq $0, %rax
  movq %rax, -24(%rbp)
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq $20, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb whilebody
  jmp whileend

whilebody:
  movq -24(%rbp), %rax
  pushq %rax
  movq -16(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -24(%rbp)
  movq -8(%rbp), %rax
  pushq %rax
  movq $3, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  movq -24(%rbp), %rax
  pushq %rax
  movq $50, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen
  jmp ifelse

ifthen:
  movq -16(%rbp), %rax
  pushq %rax
  movq $1, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  jmp ifend

ifelse:
  movq -16(%rbp), %rax
  pushq %rax
  movq $2, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  jmp ifend

ifend:
  jmp whilecond

whileend:
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen1
  jmp ifelse1

ifthen1:
  movq -24(%rbp), %rax
  pushq %rax
  movq %rdi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -32(%rbp)
  jmp ifend1

ifelse1:
  movq -24(%rbp), %rax
  pushq %rax
  movq %rsi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -32(%rbp)
  jmp ifend1

ifend1:
  movq -32(%rbp), %rax
  pushq %rax
  movq %rdx, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, -8(%rbp)
  movq -8(%rbp), %rax
  addq $32, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits