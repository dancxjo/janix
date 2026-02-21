Feature: Graph Personalization

  As a user
  I want to personalize the system graph by adding nodes and relationships
  So that I can model my own data structures within the OS

  Scenario: Creating and Linking Users
    Given the machine is running
    And the anther server is ready

    # 1. Create two users and a friendship link in one query
    # This verifies the fix for MERGE with inline node creation
    When I execute the GQL query "MERGE (alice:user.User {name: 'Alice'})-[:FRIEND]->(bob:user.User {name: 'Bob'}) RETURN alice, bob"
    Then the GQL result should have success = true
    And the GQL result should have 1 rows
    And row 0 column "alice" of the GQL result should contain "Alice"
    And row 0 column "bob" of the GQL result should contain "Bob"
    And I save the ID of row 0 column "alice" as "ALICE_ID"
    And I save the ID of row 0 column "bob" as "BOB_ID"

    # 2. Verify Alice exists independently
    When I execute the GQL query "MATCH (u) WHERE id(u) = {{ALICE_ID}} RETURN u"
    Then the GQL result should have success = true
    And row 0 column "u" of the GQL result should contain "Alice"

    # 3. Verify the link exists
    # We query for the relationship between the specific IDs
    # Note: We rely on the fact that MATCH (a)-...->(b) implicitly filters by finding paths
    # But phloem executor MATCH logic is basic.
    # It does: `for src_id in source_nodes ... get_outbound_edges ... check dst`
    # source_nodes comes from `discover_nodes`.
    # `src` pattern is `(a)`. `src.props` is empty. `discover_nodes` will return ALL nodes if no props/kind!
    # That's too slow/heavy.
    # However, `execute_match` has an optimization:
    # "OPTIMIZATION: If WHERE id(n) = $id ... just look up that node"
    # But that optimization only applies to `Pattern::Node`.
    # `Pattern::Edge` handling:
    # `let src_id_opt = if let Some(ref props) = src.props.first() { if props.0 == "id" ... }`
    # It checks for "id" property in the pattern props! Not WHERE clause.
    # So `MATCH (a {id: 123})` works.
    # Does `phloem` support `{id: ...}` syntax for ID lookup?
    # `parse_node_pattern` parses props. `id` is just a prop.
    # But `Graph` trait doesn't have `id` as a property. ID is the handle.
    # `executor.rs`: `resolve_value_as_u64`.
    # `execute_match`: `let src_id_opt = ... if props.0 == "id"`.
    # So if I write `MATCH (a {id: {{ALICE_ID}}})`, it should optimize!

    # Let's try to use that syntax for efficiency.
    When I execute the GQL query "MATCH (a {id: {{ALICE_ID}}})-[r:FRIEND]->(b {id: {{BOB_ID}}}) RETURN r"
    Then the GQL result should have success = true
    And the GQL result should have 1 rows
    And row 0 column "r" of the GQL result should contain "FRIEND"
