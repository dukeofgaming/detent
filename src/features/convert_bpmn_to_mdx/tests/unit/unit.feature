Feature: Convert BPMN to MDX unit tests
  As a developer
  I want adapter types to parse and serialize correctly in isolation
  So that higher layers can rely on them

  Scenario: Parse MDX file
    Given the hello-world MDX file "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"
    When I parse the MDX file
    Then the MDX frontmatter contains "id: _1E892844-423C-464F-ADC4-22F1EC73851B"
    And the MDX body contains "Start Event"

  Scenario: Parse start event
    Given the hello-world MDX file "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"
    When I parse the MDX file as a start event
    Then the parsed id is "_1E892844-423C-464F-ADC4-22F1EC73851B"

  Scenario: Parse end event
    Given the hello-world MDX file "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx"
    When I parse the MDX file as an end event
    Then the parsed id is "_D3F6E97D-7783-492C-98CE-57EC815D304C"

  Scenario: Parse task
    Given the hello-world MDX file "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"
    When I parse the MDX file as a task
    Then the parsed task id is "_808AA40C-EAA1-40C4-A2DC-27000FBF1866"
    And the parsed task name is "Hello World"

  Scenario: Parse task with documentation
    Given the hello-world MDX file "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"
    When I parse the MDX file as a task
    Then the parsed task has documentation

  Scenario: Parse sequence flow start to task
    Given the hello-world MDX file "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx"
    When I parse the MDX file as a sequence flow
    Then the sequence flow connects "_1E892844-423C-464F-ADC4-22F1EC73851B" to "_808AA40C-EAA1-40C4-A2DC-27000FBF1866"

  Scenario: Parse sequence flow task to end
    Given the hello-world MDX file "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx"
    When I parse the MDX file as a sequence flow
    Then the sequence flow connects "_808AA40C-EAA1-40C4-A2DC-27000FBF1866" to "_D3F6E97D-7783-492C-98CE-57EC815D304C"

  Scenario: Roundtrip start event
    Given the hello-world MDX file "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"
    When I roundtrip the MDX file as a start event

  Scenario: Roundtrip task
    Given the hello-world MDX file "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"
    When I roundtrip the MDX file as a task

  Scenario: Roundtrip sequence flow
    Given the hello-world MDX file "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx"
    When I roundtrip the MDX file as a sequence flow

  Scenario: All hello-world MDX files parseable
    Given all hello-world per-element MDX fixtures
    When I parse each MDX file
    Then all MDX files parse successfully

  Scenario: Flow node id
    Given the hello-world BPMN XML for flow node tests
    When I inspect flow node ids from the parsed process
    Then flow node ids and types match hello-world

  Scenario: Flow node from sequence flow
    Given the hello-world BPMN XML for flow node tests
    When I inspect the first sequence flow
    Then the sequence flow has non-empty id and refs
