Feature: Kernel Multi-Threading
  As a kernel developer
  I want to verify that multiple threads can run and cooperate
  So that I can build more complex system services

  Scenario: Threads A and B interleave
    Given the machine is booted
    Then I should see "Spawning Thread A..."
    And I should see "Spawning Thread B..."
    And I should see "Thread A (arg=0) ticks="
    And "Thread A (arg=0) ticks=" should appear at least 2 times
    And I should see "Thread B (arg=0) ticks="
    And "Thread B (arg=0) ticks=" should appear at least 2 times
    And I should see "Thread A (arg=0) ticks=" after "Thread B (arg=0) ticks="
