@boot
Feature: Boot Screen Colors

  Scenario: Lilac Boot Screen
    Given the machine is started
    When I wait for the system to boot
    Then the screen should be filled with "Lilac"
