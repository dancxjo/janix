@boot @smoke
Feature: System Boot and UI Bring-Up

  As a system developer
  I want to verify that ThingOS boots successfully
  So that I can confirm the kernel, graphics, compositor, and UI are fully functional

  Scenario: Boot produces logs, graphics, and a live desktop
    When I start the machine
    Then I should see log messages on the terminal
    And each log message should include a monotonically increasing timestamp
    And I should see the system clock tick for several seconds in the serial console
    And I should see the wallpaper on the screen within 60 seconds
    And I should see a cursor centered on the screen
    And I should see the text "thing-os" in the top-left corner of the screen
    And I should see frame count information in the top-left corner of the screen
    And I should see a clock window displaying a ticking clock
