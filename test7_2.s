.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $16, %rsp
  movq $3, %rax
  movq %rax, -8(%rbp)
  movq $10, %rax
  movq %rax, -16(%rbp)
  movq -16(%rbp), %rax
  pushq %rax
  movq -8(%rbp), %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen
  jmp ifelse

ifthen:
  movq $0, %rax
  movq %rax, -16(%rbp)
  jmp ifend

ifelse:
  movq $0, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifend:
  movq -16(%rbp), %rax
  pushq %rax
  movq %rdi, %rax
  popq %rbx
  addq %rbx, %rax
  pushq %rax
  movq %rdi, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, -16(%rbp)
  movq -16(%rbp), %rax
  addq $16, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits