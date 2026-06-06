Feature: Convert BPMN to MDX
  As a developer
  I want to round-trip between BPMN XML and MDX flow-element files
  So that I can author processes in MDX and emit valid BPMN

  Scenario: Importing the hello-world BPMN produces one MDX file per flow element
    Given the hello-world BPMN fixture
    When I import it to MDX
    Then 5 MDX outputs are produced
    And each output contains a frontmatter block

  Scenario: Compiling a minimal MDX set produces a valid Definitions with a process
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    Then a process is produced with 1 start event, 1 task, 1 end event, and 2 sequence flows

  Scenario: Compiling rejects empty input
    Given no MDX inputs
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Compiling rejects MDX missing the type field
    Given an MDX input whose frontmatter omits the type field
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Feature-3 compile tolerates a dangling sequence flow target
    Given an MDX input set where a sequence flow targets a non-existent node
    When I compile the MDX inputs to definitions
    Then compilation succeeds
    And the resulting process contains the dangling target id
