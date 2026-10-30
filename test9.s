.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  subq $48, %rsp
  movq $5, %rax
  addq $7, %rax
  movq %rax, -8(%rbp)
  movq $5, %rax
  imulq $5, %rax
  movq %rax, -16(%rbp)
  movq -8(%rbp), %rax
  addq -16(%rbp), %rax
  movq %rax, -24(%rbp)
  movq $7, %rax
  addq $7, %rax
  movq %rax, -32(%rbp)
  movq -24(%rbp), %rax
  addq -32(%rbp), %rax
  movq %rax, -40(%rbp)
  movq -40(%rbp), %rax
  addq -32(%rbp), %rax
  movq %rax, -48(%rbp)
  movq -48(%rbp), %rax
  movq %rbp, %rsp
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits