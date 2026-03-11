Feature: Personal Knowledge Graph

  As a knowledge worker
  I want to create and connect nodes in the system graph
  So that I can build a personal knowledge base directly in the OS

  Scenario: Creating a new thought node
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MERGE (t:sys.Thought {content: 'ThingOS is a graph'}) RETURN t"
    Then the response status should be 200
    And the response body should contain "ThingOS is a graph"

  Scenario: Connecting thoughts together
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MERGE (a:sys.Thought {content: 'Idea 1'})-[:RELATES_TO]->(b:sys.Thought {content: 'Idea 2'}) RETURN a, b"
    Then the response status should be 200
    And the response body should contain "Idea 1"
    And the response body should contain "Idea 2"
