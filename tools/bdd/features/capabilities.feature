Feature: Capabilities and least privilege
  Syscalls that mutate or access sensitive resources require capabilities.
  Services start with minimal rights and must be explicitly granted more.

  @caps @wip
  Scenario: A process cannot map memory without a mapping capability
    Given sprout is online
    And the process "logview" has no capability "cap.map_memory"
    When "logview" attempts to map a bytespace into its address space
    Then the syscall should fail with status "EPERM"

  @caps @wip
  Scenario: A process cannot read raw input without an input capability
    Given sprout is online
    And the process "logview" has no capability "cap.read_input"
    When "logview" attempts to read from the input device
    Then the syscall should fail with status "EPERM"

  @caps @framebuffer @wip
  Scenario: Framebuffer details are not exposed without explicit capability
    Given sprout is online
    And the process "bloom" has no capability "cap.framebuffer"
    When "bloom" requests framebuffer metadata
    Then the syscall should fail with status "EPERM"

  @caps @insecure @wip
  Scenario: Insecure escape hatches are feature-flagged
    Given the kernel is built without the feature "insecure_dev"
    When a process attempts to use an insecure syscall
    Then the syscall should fail with status "ENOSYS"
