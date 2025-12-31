Feature: Boot milestones

  Scenario Outline: OS boots successfully
    Given I have an iso for "<arch>"
    When I boot
    Then I should see "Booted." in the serial console

    Examples:
      | arch    |
      | x86_64  |
      | aarch64 |
