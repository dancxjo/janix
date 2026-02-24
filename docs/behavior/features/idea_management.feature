Feature: Idea Management

  As a creative user
  I want to record and organize my ideas in the system graph
  So that I can develop them later

  Scenario: Creating and Linking Ideas
    Given the machine is running
    And the anther server is ready

    # Create the entire structure in one go
    # We use a unique name to avoid conflicts if test runs multiple times (though graph state resets usually)
    When I execute the GQL query "MERGE (p:Project {name: \"ThingOS Next\"})-[:CONTAINS]->(i:Idea {title: \"Graph Filesystem\", status: \"Draft\"}) RETURN p"
    Then the response status should be 200

    # Verify Project exists
    When I execute the GQL query "MATCH (p:Project {name: \"ThingOS Next\"}) RETURN p"
    Then the response status should be 200
    And the response body should contain "ThingOS Next"

    # Verify Idea exists
    When I execute the GQL query "MATCH (i:Idea {title: \"Graph Filesystem\"}) RETURN i"
    Then the response status should be 200
    And the response body should contain "Graph Filesystem"

    # Verify Link
    When I execute the GQL query "MATCH (p:Project)-[:CONTAINS]->(i:Idea) RETURN i"
    Then the response status should be 200
    And the response body should contain "Graph Filesystem"
