Feature: User Mode v0
  As a kernel developer
  I want to verify that user mode tasks can execute and perform syscalls
  So that I can build userspace applications

  Scenario: User mode stub executes and yields
    Given a running system
    Then the log should contain "user: entered"
    And the log should contain "syscall: putchar"
    And the log should contain "syscall: yield"
