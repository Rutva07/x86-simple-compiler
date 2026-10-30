.text
  .global foo

foo:
  pushq %rbp
  movq %rsp, %rbp
  pushq %rbx
  subq $8, %rsp
  movq $0, %rax
  movq %rax, -8(%rbp)
  movq %rdi, %rax
  pushq %rax
  movq $5, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jae ifthen
  jmp ifelse

ifthen:
  movq $5, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifelse:
  movq $0, %rax
  movq %rax, -8(%rbp)
  jmp ifend

ifend:
  movq %rsi, %rax
  pushq %rax
  movq $5, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  ja ifthen1
  jmp ifelse1

ifthen1:
  movq -8(%rbp), %rax
  pushq %rax
  movq $5, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend1

ifelse1:
  movq -8(%rbp), %rax
  pushq %rax
  movq $5, %rax
  popq %rbx
  imulq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend1

ifend1:
  movq %rdx, %rax
  pushq %rax
  movq $5, %rax
  movq %rax, %rbx
  popq %rax
  cmpq %rbx, %rax
  jb ifthen2
  jmp ifelse2

ifthen2:
  movq -8(%rbp), %rax
  pushq %rax
  movq $1, %rax
  movq %rax, %rbx
  popq %rax
  subq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend2

ifelse2:
  movq -8(%rbp), %rax
  pushq %rax
  movq $1, %rax
  popq %rbx
  addq %rbx, %rax
  movq %rax, -8(%rbp)
  jmp ifend2

ifend2:
  movq -8(%rbp), %rax
  addq $8, %rsp
  popq %rbx
  popq %rbp
  ret

.section .note.GNU-stack,"",@progbits