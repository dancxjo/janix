Feature: System Philosophy

  As a curious user
  I want to understand the "Everything is a Thing" philosophy of Thing-OS
  So that I can effectively interact with the system graph and desktop

  Scenario: Discovering that visual elements are graph nodes
    Given the machine is running
    And the anther server is ready
    When I wait for 5 seconds
    Then I should see the desktop wallpaper
    And I should see the "Photosynthesis" application window
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n.name"
    Then the response status should be 200
    And the response body should contain "bloom"
    And the response body should contain "photosynthesis"

  Scenario: Understanding the system root
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:svc.Root) RETURN n"
    Then the response status should be 200
    And the response body should contain "svc.Root"
