.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $24, %rsp
  movq $0, %rax
  movq %rax, -8(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen
  jmp ifelse

ifthen:
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, -24(%rbp)
  jmp ifend

ifelse:
  movq %rsi, %rax
  pushq %rax
  movq %rdi, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, -24(%rbp)
  jmp ifend

ifend:
  movq -24(%rbp), %rax
  pushq %rax
  movq $10, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb ifthen1
  jmp ifelse1

ifthen1:
  movq -24(%rbp), %rax
  pushq %rax
  movq $5, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend1

ifelse1:
  movq -24(%rbp), %rax
  pushq %rax
  movq $15, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend1

ifend1:
  movq -8(%rbp), %rax
  addq $24, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits