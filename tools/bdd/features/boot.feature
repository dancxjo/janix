Feature: Boot Screen Colors

  Scenario: Mallard Teal Boot Screen
    Given the machine is started
    When I wait for the system to boot
    Then the screen should be filled with "Mallard Teal"
