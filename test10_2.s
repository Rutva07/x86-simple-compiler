.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $24, %rsp
  movq $1, %rax
  movq %rax, -8(%rbp)
  movq $2, %rax
  movq %rax, -16(%rbp)
  movq $0, %rax
  movq %rax, -24(%rbp)
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq $4, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb whilebody
  jmp whileend

whilebody:
  movq $1, %rax
  movq %rax, -16(%rbp)
  jmp whilecond1

whilecond1:
  movq -16(%rbp), %rax
  pushq %rax
  movq $3, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb whilebody1
  jmp whileend1

whilebody1:
  movq -24(%rbp), %rax
  pushq %rax
  movq -8(%rbp), %rax
  pushq %rax
  movq -16(%rbp), %rax
  popq %rbx
  imulq %rbx, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -24(%rbp)
  movq -16(%rbp), %rax
  pushq %rax
  movq $1, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  jmp whilecond1

whileend1:
  movq -8(%rbp), %rax
  pushq %rax
  movq $1, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp whilecond

whileend:
  movq -24(%rbp), %rax
  pushq %rax
  movq $10, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen
  jmp ifelse

ifthen:
  movq %rsi, %rax
  pushq %rax
  movq -24(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, %rsi
  jmp ifend

ifelse:
  movq %rsi, %rax
  pushq %rax
  movq $1, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, %rsi
  jmp ifend

ifend:
  movq %rsi, %rax
  addq $24, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits