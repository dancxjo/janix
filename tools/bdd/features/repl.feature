Feature: Graph REPL over Serial (OpenGQL)
Feature: Interactive Graph REPL
  As a developer
  I want an interactive REPL that accepts OpenGQL queries over serial
  So that I can inspect and manipulate the system graph live

Background: REPL startup
Background:
  Given the system has booted successfully
  And the system graph is initialized
  And a serial console is available
  When the graph REPL is started

Scenario: REPL announces readiness
Scenario: REPL announces readiness
  Then the serial output should contain "graph>"
  And the REPL state should be idle
  And a Thing of kind ReplSession should exist in the graph

Scenario: Empty input does nothing
Scenario: Empty input is ignored
  When the user enters an empty line
  Then no graph mutation should occur
  And the prompt "graph>" should be shown again

Scenario: Simple MATCH query
Scenario: Query nodes using MATCH
  When the user enters:
    """
    MATCH (n) RETURN n LIMIT 1;
    """
  Then the REPL should parse the query successfully
  And the query should be recorded as a Thing of kind Query
  And the serial output should contain a serialized node result
  And the prompt "graph>" should be shown again

Scenario: Query returns no results
Scenario: Query with no matching results
  When the user enters:
    """
    MATCH (n:DefinitelyNotReal) RETURN n;
    """
  Then the query should execute successfully
  And the serial output should contain "0 results"
  And no error Thing should be created

Scenario: Create a node
Scenario: Create a Thing using CREATE
  When the user enters:
    """
    CREATE (t:Thing { kind: "TestNode" });
    """
  Then a new Thing should be added to the graph
  And the Thing should have kind "TestNode"
  And the serial output should confirm creation

Scenario: Create a relationship
Scenario: Create a relationship between Things
  Given a Thing with kind "A" exists
  And a Thing with kind "B" exists
  When the user enters:
    """
    MATCH (a:Thing {kind:"A"}), (b:Thing {kind:"B"})
    CREATE (a)-[:LINKS_TO]->(b);
    """
  Then a Relationship Thing should exist in the graph
  And the relationship type should be "LINKS_TO"

Scenario: Syntax error handling
Scenario: Invalid OpenGQL syntax
  When the user enters:
    """
    METCH (n RETURN n;
    """
  Then the REPL should reject the query
  And a Thing of kind QueryError should be created
  And the serial output should contain "syntax error"
  And the prompt "graph>" should be shown again

Scenario: Semantic error handling
Scenario: Valid syntax but invalid semantics
  When the user enters:
    """
    MATCH (n) CREATE n;
    """
  Then the REPL should fail execution
  And a Thing of kind QueryError should exist
  And the error should reference semantic validation

Scenario: Multi-line query input
Scenario: Multi-line OpenGQL query
  When the user enters:
    """
    MATCH (n)
    WHERE n.kind = "Task"
    RETURN n;
    """
  Then the REPL should accept multi-line input
  And the query should execute successfully
  And the results should be printed over serial

Scenario: Query history is preserved
Scenario: Query history is recorded
  When the user enters:
    """
    MATCH (n) RETURN n LIMIT 1;
    """
  Then a Thing of kind Query should exist
  And the Query Thing should reference the ReplSession
  And the query text should be stored verbatim

Scenario: REPL exit command
Scenario: Exit the REPL
  When the user enters:
    """
    :exit
    """
  Then the REPL should terminate cleanly
  And the ReplSession Thing should be marked closed
  And control should return to the serial shell

Scenario: Unknown command
Scenario: Unknown REPL command
  When the user enters:
    """
    :dance
    """
  Then the serial output should contain "unknown command"
  And no graph mutation should occur
