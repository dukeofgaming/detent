Feature: Graph validation integration tests
  As a developer
  I want graph validation to work with real BPMN fixtures
  So that imported processes are validated correctly

  Scenario Outline: Well-formed BPMN passes graph validation
    Given the <fixture> BPMN fixture
    When I validate the process graph
    Then validation succeeds

    Examples:
      | fixture    |
      | linear     |
      | branching  |

  Scenario Outline: Fixture flow is end-to-end
    Given the <fixture> BPMN fixture
    When I analyze the process flow
    Then the entry node is <entry>
    And the exit node is <exit>
    And <entry> can reach <exit>

    Examples:
      | fixture    | entry    | exit     |
      | linear     | start_1  | end_1    |
      | branching  | start_1  | end_1    |
