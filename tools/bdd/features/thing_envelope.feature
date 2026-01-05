Feature: Kernel-Enforced Thing Body Envelopes
  The kernel must ensure that all Thing bodies follow a standard envelope format
  and verify their structural integrity at runtime.

  @graph @envelope @smoke
  Scenario: Kernel rejects malformed Thing bodies
    Given the system has reached "kernel ready"
    When I attempt to create a Thing with an invalid magic number
    Then the kernel should return ERR_INVALID_THING_BODY

  @graph @envelope @smoke
  Scenario: Kernel computes and returns integrity digests
    Given the system has reached "kernel ready"
    When I create a Thing with a valid envelope
    Then I should be able to retrieve it and get a matching CRC64 digest

  @graph @envelope @smoke
  Scenario: Things are self-describing without a global ontology
    Given the system has reached "userland start"
    And thingcheck is running
    Then thingcheck should successfully decode a DisplayDevice body from the graph
    And it should not require a global ontology.bin
