Feature: Graph Validation
  As a process modeler
  I want structural validation of BPMN process graphs
  So that I can catch broken processes before runtime

  Scenario: A well-formed linear process passes validation
    Given a linear process with one start event, one task, and one end event
    When I validate the process graph
    Then validation succeeds

  Scenario: A process with no start event is rejected
    Given a linear process with no start event
    When I validate the process graph
    Then validation fails reporting "start event"

  Scenario: A process with no end event is rejected
    Given a linear process with no end event
    When I validate the process graph
    Then validation fails reporting "end event"

  Scenario: A sequence flow pointing at a missing node is reported
    Given a linear process whose first sequence flow targets a missing node "ghost_task"
    When I validate the process graph
    Then validation fails reporting "ghost_task"

  Scenario: Duplicate node ids are rejected
    Given a linear process whose task reuses the start event id "start_1"
    When I validate the process graph
    Then validation fails reporting "Duplicate"

  Scenario: An unreachable orphan node is detected
    Given a linear process with an orphan task "orphan_1"
    When I validate the process graph
    Then validation fails reporting "orphan_1"
