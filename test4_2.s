.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $16, %rsp
  movq $0, %rax
  movq %rax, -16(%rbp)
  movq $5, %rax
  movq %rax, -8(%rbp)
  jmp whilecond

whilecond:
  movq -8(%rbp), %rax
  pushq %rax
  movq $0, %rax
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
  jmp whilecond1

whilecond1:
  movq %rdi, %rax
  pushq %rax
  movq $0, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja whilebody1
  jmp whileend1

whilebody1:
  movq -16(%rbp), %rax
  pushq %rax
  movq $100, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -16(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq $1, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, %rdi
  jmp whilecond1

whileend1:
  movq -16(%rbp), %rax
  addq $16, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits