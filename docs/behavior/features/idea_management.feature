Feature: Idea Management

  As a user
  I want to use the system graph to manage my ideas
  So that I can organize my thoughts natively in the OS

  Scenario: Creating and Linking Ideas
    Given the anther server is ready

    # Create first idea
    When I execute the GQL query "CREATE (n:Idea {title: \"Idea 1\"})"
    Then the response status should be 200
    And the response body should contain "success"

    # Create second idea
    When I execute the GQL query "CREATE (n:Idea {title: \"Idea 2\"})"
    Then the response status should be 200

    # Link them
    When I execute the GQL query "MATCH (a:Idea {title: \"Idea 1\"}), (b:Idea {title: \"Idea 2\"}) CREATE (a)-[:RELATES_TO]->(b)"
    Then the response status should be 200
    And the response body should contain "edges"

    # Verify the link
    When I execute the GQL query "MATCH (a:Idea {title: \"Idea 1\"})-[:RELATES_TO]->(b:Idea {title: \"Idea 2\"}) RETURN a"
    Then the response status should be 200
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node

    # Verify relationship type
    When I execute the GQL query "MATCH (a:Idea {title: \"Idea 1\"})-[r]->(b:Idea {title: \"Idea 2\"}) RETURN r"
    Then the response status should be 200
    And row 0 column 0 of the GQL result should be string "RELATES_TO"
