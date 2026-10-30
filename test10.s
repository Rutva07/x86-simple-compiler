.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  subq $64, %rsp
  movq %rsi, %rax
  imulq %rdx, %rax
  movq %rax, -8(%rbp)
  movq -8(%rbp), %rax
  imulq %rdi, %rax
  movq %rax, -16(%rbp)
  movq %rdi, %rax
  addq -16(%rbp), %rax
  movq %rax, -24(%rbp)
  movq -24(%rbp), %rax
  addq %rcx, %rax
  movq %rax, -32(%rbp)
  movq %r8, %rax
  imulq %rdi, %rax
  movq %rax, -40(%rbp)
  movq -40(%rbp), %rax
  imulq %rdx, %rax
  movq %rax, -48(%rbp)
  movq -48(%rbp), %rax
  addq %rdx, %rax
  movq %rax, -56(%rbp)
  movq -32(%rbp), %rax
  addq -56(%rbp), %rax
  movq %rax, -64(%rbp)
  movq -64(%rbp), %rax
  movq %rbp, %rsp
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits