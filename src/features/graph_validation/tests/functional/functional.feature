Feature: Graph validation functional tests
  As a process modeler
  I want structural validation of BPMN process graphs
  So that I can catch broken processes before runtime

  Scenario: Well-formed process passes
    Given a well-formed linear process
    When I validate the process graph
    Then validation succeeds

  Scenario: No start event rejected
    Given a linear process with no start event
    When I validate the process graph
    Then validation fails reporting "start event"

  Scenario: No end event rejected
    Given a linear process with no end event
    When I validate the process graph
    Then validation fails reporting "end event"

  Scenario: Dangling target reported
    Given a linear process with a dangling target "ghost_task"
    When I validate the process graph
    Then validation fails reporting "ghost_task"

  Scenario: Duplicate id rejected
    Given a linear process with a duplicate id "start_1"
    When I validate the process graph
    Then validation fails reporting "Duplicate"

  Scenario: Orphan node detected
    Given a linear process with an orphan "orphan_1"
    When I validate the process graph
    Then validation fails reporting "orphan_1"

  Scenario: Dangling source reported
    Given a linear process with a dangling source "ghost"
    When I validate the process graph
    Then validation fails reporting "ghost"

  Scenario: Duplicate flow id rejected
    Given a linear process with a duplicate flow id "flow_1"
    When I validate the process graph
    Then validation fails reporting "Duplicate"

  Scenario: Dead end detected
    Given a linear process with a dead end "nowhere"
    When I validate the process graph
    Then validation fails reporting "dead end"

  Scenario: Flow connectivity
    Given a well-formed linear process
    When I analyze the process flow
    Then start_1 flows to task_1
    And task_1 flows to end_1
    And end_1 has no outgoing flows
    And start_1 has no incoming flows
    And task_1 comes from start_1
    And end_1 comes from task_1
    And the entry node is start_1
    And the exit node is end_1

  Scenario: Reachable nodes
    Given a well-formed linear process
    When I analyze the process flow
    Then start_1 can reach task_1
    And start_1 can reach end_1
    And task_1 can reach end_1

  Scenario: Orphan is isolated
    Given a linear process with an orphan "orphan_1"
    When I analyze the process flow
    Then orphan_1 can only reach itself

  Scenario: Branching process passes validation
    Given a branching process with an exclusive gateway
    When I validate the process graph
    Then validation succeeds

  Scenario: Branch and merge connectivity
    Given a branching process with an exclusive gateway
    When I analyze the process flow
    Then gateway_1 branches to task_a and task_b
    And end_1 merges task_a and task_b
    And start_1 can reach task_a
    And start_1 can reach task_b
    And start_1 can reach end_1
