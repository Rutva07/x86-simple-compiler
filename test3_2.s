.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $56, %rsp
  movq $1, %rax
  movq %rax, -16(%rbp)
  movq $2, %rax
  movq %rax, -24(%rbp)
  movq $3, %rax
  movq %rax, -32(%rbp)
  movq $4, %rax
  movq %rax, -40(%rbp)
  movq $5, %rax
  movq %rax, -48(%rbp)
  movq $6, %rax
  movq %rax, -56(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq %rsi, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen
  jmp ifelse

ifthen:
  movq -16(%rbp), %rax
  pushq %rax
  movq -24(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  pushq %rax
  movq -32(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifelse:
  movq -40(%rbp), %rax
  pushq %rax
  movq -48(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  pushq %rax
  movq -56(%rbp), %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifend:
  movq -8(%rbp), %rax
  addq $56, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits