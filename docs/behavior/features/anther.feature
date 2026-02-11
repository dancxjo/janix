Feature: Anther HTTP Server

  Scenario: Server starts up
    When I start the machine
    Then I should see a message in the serial output that says "anther: Listening on port 80"

  Scenario: Serve serial requests
    Given the anther server is ready
    When I make a GET request to "/health"
    Then the response status should be 200
    And the response body should contain "ok"
    When I make a GET request to "/graph"
    Then the response status should be 200
    And the response body should contain "Graph index"

  Scenario: Serve concurrent requests
    Given the anther server is ready
    When I make 10 concurrent GET requests to "/health"
