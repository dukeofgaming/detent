Feature: Graph Validation
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
