@tools
Feature: System Tools

  As a system administrator
  I want to use utility applications
  So that I can monitor system activity and inspect media

  Scenario: Task Manager displays the process list
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see a message in the serial output that says "TASKMAN: starting task manager" within 300s
    And the serial output should contain "TASKMAN: Window created"
    # Task Manager window at 8, 30 with size 500x400 and color #F0F0F0 (semi-transparent)
    And I should see a rectangle at 8, 30 with size 500x400 and color "#F0F0F0"
    # Check for text-like pixels inside the window (e.g. title bar or list items)
    And I should see text-like pixels inside the window at 20, 50

  Scenario: ISO Reader scans the boot media
    Given the machine is started
    When I wait for the system to reach ready state
    Then the serial output should contain "ISO_READER: Starting ISO9660 reader service"
    And the serial output should contain "ISO_READER: Found ISO9660 filesystem"
    # Wait for scan completion (might take a moment depending on ISO size)
    And I should see a message in the serial output that says "ISO_READER: Scan complete" within 120s
