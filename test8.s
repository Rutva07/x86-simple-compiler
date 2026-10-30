.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  subq $24, %rsp
  movq $5, %rax
  addq %rdi, %rax
  movq %rax, -8(%rbp)
  movq -8(%rbp), %rax
  addq $10, %rax
  movq %rax, -16(%rbp)
  movq -16(%rbp), %rax
  imulq -16(%rbp), %rax
  movq %rax, -24(%rbp)
  movq -24(%rbp), %rax
  movq %rbp, %rsp
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits