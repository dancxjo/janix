@torture @stacks @heaps
Feature: Stacks and heaps are safe under stress
  As a kernel developer
  I want to verify memory safety under stress
  So that corruption is caught early, loudly, and repeatably

  Scenario: User stack grows safely under deep recursion
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see "PASS: stack_recursion"
    And the log does not contain "FAIL: stack_recursion"
    And the log does not contain "stack canary tripped"

  Scenario: Stack-local patterns survive context switches
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see "PASS: stack_context_stress"
    And the log does not contain "FAIL: stack_context_stress"

  Scenario: Heap allocations survive churn
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see "PASS: heap_churn"
    And the log does not contain "FAIL: heap_churn"

  Scenario: Heap realloc preserves prefix data
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see "PASS: heap_realloc"
    And the log does not contain "FAIL: heap_realloc"

  Scenario: Heap fuzz is stable for a fixed seed
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see "PASS: heap_fuzz seed=123"
    And the log does not contain "FAIL: heap_fuzz"
