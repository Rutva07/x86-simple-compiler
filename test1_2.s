.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $16, %rsp
  movq $0, %rax
  movq %rax, -16(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq $4, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb ifthen
  jmp ifelse

ifthen:
  movq $4, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifelse:
  movq $6, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifend:
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq $4, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja whilebody
  jmp whileend

whilebody:
  movq -16(%rbp), %rax
  pushq %rax
  movq %rdi, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  movq -8(%rbp), %rax
  pushq %rax
  movq $1, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp whilecond

whileend:
  movq -16(%rbp), %rax
  addq $16, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits