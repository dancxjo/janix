Feature: Scheduler is a Place
  The scheduler is not an opaque kernel mechanism.
  It exists as a Place in the system graph, containing Tasks as Things,
  with task state expressed as Relationships.

  Scenario: The scheduler exists as a Place in the system graph
    Given the system has completed kernel initialization
    When I inspect the system graph
    Then a Place named "place.scheduler" must exist
    And that Place must be queryable like any other Place

  Scenario: The initial task exists as a Thing in the scheduler
    Given the scheduler Place exists
    When the system begins executing userland
    Then a Thing named "thing.task.sprout" must exist
    And that Thing must be contained within "place.scheduler"

  Scenario: Task state is expressed as a relationship, not a field
    Given a Task Thing named "thing.task.sprout"
    When the task is actively executing
    Then there must exist a Relationship from "thing.task.sprout"
    And the predicate of that Relationship must be "predicate.state"
    And the target of that Relationship must be "state.running"

  Scenario: Task state transitions leave a trace in the graph
    Given a Task Thing named "thing.task.sprout" is running
    When the task yields execution
    Then the Relationship expressing "state.running" must no longer be present
    And a Relationship expressing "state.blocked" must exist instead

  Scenario: The scheduler can be inspected without affecting execution
    Given the system is running normally
    When I query the scheduler Place
    Then I must be able to list all Task Things it contains
    And for each Task I must be able to determine its current state
    And performing this query must not alter task execution

  Scenario: The scheduler describes reality even in a single-task system
    Given the system supports only one executing task
    When I inspect the scheduler Place
    Then exactly one Task Thing must be present
    And its state must be "state.running"

  Scenario: Task identity persists across execution changes
    Given a Task Thing named "thing.task.sprout"
    When the task transitions between running and blocked states
    Then the identity of the Task Thing must remain the same

  Scenario: Scheduling policy is not embedded in task identity
    Given a Task Thing exists in the scheduler
    When I inspect its relationships
    Then no Relationship must imply how the scheduler chooses it
    And scheduling policy must be derivable from relationships, not hardcoded rules

  Scenario: The scheduler world exists before userland acts
    Given the kernel has completed initialization
    When no userland code has yet executed
    Then the scheduler Place must already exist
    And it must already contain the initial Task Thing

  Scenario: The scheduler Place can be extended without changing the kernel
    Given the scheduler Place exists
    When a new Task Thing is added to that Place
    Then the scheduler must be able to observe it
    And the kernel must not require recompilation to understand the new Task

  Scenario: Scheduler facts can be recorded without enforcing behavior
    Given a Task Thing has a Relationship expressing "state.blocked"
    When the kernel continues executing another task
    Then the blocked state must remain a fact in the graph
    And even if no behavior depends on it yet

  Scenario: Scheduling is a narrative, not a black box
    Given the system has been running for some time
    When I inspect the scheduler Place
    Then I must be able to explain why each Task is running or not running
    And using only the facts expressed in the graph

  Scenario: The scheduler Place is just another Place
    Given the system graph contains many Places
    When I compare "place.scheduler" with "place.desktop"
    Then both must support containment
    And both must support query
    And neither must require special-case logic to inspect

  Scenario: Scheduling is made visible
    Given the system is running
    When I ask "what is happening right now?"
    Then the answer must be derivable from the graph
    And the scheduler must not be exempt from that answer
