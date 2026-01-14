Feature: Asset Ingest
  Scenario: Ingestd detects PNG content
    Given the machine is booted
    Then the serial output should contain "SPROUT: Launching app '/boot/ingestd'"
    And the serial output should contain "SPROUT: Launching app '/boot/png_creator'"
    And I wait for 5 seconds
    Then the serial output should contain "Ingestd: Watch event received for node"
    And the log should contain "PNG_CREATOR: Wrote PNG header"
