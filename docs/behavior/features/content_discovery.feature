@system @content
Feature: Content Discovery

  As a system integrator
  I want the system to automatically mount and scan the boot media
  So that applications can access files and assets without manual configuration

  Scenario: ISO9660 Filesystem is detected and scanned
    Given the machine is booting
    When I wait for the system to reach ready state
    Then the serial output should contain "ISO_READER: Found ATAPI device"
    And the serial output should contain "ISO_READER: Found ISO9660 filesystem"
    And the serial output should contain "ISO_READER: Scan complete"
    And the serial output should contain "ISO_READER: Published"
