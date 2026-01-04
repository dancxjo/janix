Feature: Ontology system coherence
  The ontology defines the meaning of Things and is consistent across
  authoring, runtime, and graph usage.

  Background:
    Given the workspace has generated ontology artifacts
    And the system is running with serial logging enabled

  @ontology @host
  Scenario: Ontology generation is deterministic
    When I run the ontology generation twice
    Then the ontology digest is identical
    And the generated symbols file is identical
    And the generated model glue file is identical

  @ontology @registry
  Scenario: Ontology registry is available at runtime
    When a userspace program fetches the ontology registry
    Then the registry size is greater than 0
    And the registry digest matches the build-time digest

  @ontology @schema
  Scenario: A known schema exists
    When a userspace program inspects the ontology registry
    Then it reports schema "schema.Surface@1" exists
    And it reports kind "kind.Surface" exists

  @ontology @roundtrip
  Scenario: A model round-trips through encoding and decoding
    When a userspace program encodes a "Surface" model
    And it decodes the resulting payload
    Then it reports "OK: model_roundtrip Surface"

  @ontology @mismatch
  Scenario: Schema mismatch is rejected
    When a userspace program attempts to decode a payload with the wrong schema
    Then it reports "OK: schema_mismatch_detected"
