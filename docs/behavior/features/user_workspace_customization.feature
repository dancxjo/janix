Feature: User Workspace Customization

  As a power user
  I want to customize my workspace using the System Graph
  So that my environment reflects my preferences

  Scenario: Setting a custom workspace theme
    Given the machine is running
    And the anther server is ready

    # Create a new theme configuration node
    When I execute the GQL query "CREATE (t:Theme {name: \"Dark Mode\", primary_color: \"#1a1a1a\"})"
    Then the response status should be 200

    # Create a user session node (if it doesn't exist)
    When I execute the GQL query "CREATE (s:Session {user: \"admin\"})"
    Then the response status should be 200

    # Link the theme to the current user or system session
    When I execute the GQL query "MATCH (s:Session {user: \"admin\"}), (t:Theme {name: \"Dark Mode\"}) CREATE (s)-[:USES_THEME]->(t)"
    Then the response status should be 200

    # Verify the theme is applied (querying it back)
    When I execute the GQL query "MATCH (s:Session)-[:USES_THEME]->(t:Theme) RETURN t"
    Then the response status should be 200
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node
