Feature: Telnet Server

  Scenario: Server starts up
    When I start the machine
    Then I should see a message in the serial output that says "telnetd: listening on guest port 2323"

  Scenario: Serve telnet connection
    Given the telnet server is ready
    When I connect to the telnet server and send "match (n) return n;"
    Then the telnet response should contain "node("
