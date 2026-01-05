Feature: System thingification
  Core subsystems are represented as typed Things with canonical kinds and schemas.

  Background:
    Given the system is running with serial logging enabled

  Scenario: Display device Things exist and are decodable
    When a userspace program checks core Things
    Then it reports "OK: graph.display present"
    And it reports "OK: DisplayDevice decodable"
    And it reports "OK: Framebuffer decodable"

  Scenario: Surface is backed by a bytespace
    When a userspace program checks core Things
    Then it reports "OK: Surface has Bytespace"

  Scenario: Mouse stream exists and pointer state mirrors it
    When a userspace program checks input Things
    Then it reports "OK: MouseStream present"
    And it reports "OK: PointerState decodable"

  Scenario: Window Things exist and are discoverable
    When a userspace program checks window Things
    Then it reports "OK: graph.windows present"
