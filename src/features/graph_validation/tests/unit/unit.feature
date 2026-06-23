Feature: Graph validation unit tests
  As a developer
  I want graph primitives to behave correctly in isolation
  So that higher-level validation can rely on them

  Scenario: Find node returns existing node
    Given a linear process graph
    When I find node "task_1"
    Then the node id is "task_1"

  Scenario: Find node returns none for missing id
    Given a linear process graph
    When I find node "nonexistent"
    Then no node is found

  Scenario: Successors of nonexistent node is empty
    Given a linear process graph
    When I list successors of "nonexistent"
    Then successors are empty
