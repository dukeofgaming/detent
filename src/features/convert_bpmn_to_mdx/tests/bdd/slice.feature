Feature: Convert BPMN to MDX
  As a developer
  I want to round-trip between BPMN XML and MDX flow-element files
  So that I can author processes in MDX and emit valid BPMN

  Scenario: Hello-world imports to 5 MDX
    Given the hello-world BPMN fixture
    When I import it to MDX
    Then 5 MDX outputs are produced
    And each output contains a frontmatter block
    And each output contains its BPMN type

  Scenario: Minimal MDX compiles to process
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    Then a process is produced with 1 start event, 1 task, 1 end event, and 2 sequence flows

  Scenario: Empty input rejected
    Given no MDX inputs
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Missing type rejected
    Given an MDX input whose frontmatter omits the type field
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Dangling flow tolerated
    Given an MDX input set where a sequence flow targets a non-existent node
    When I compile the MDX inputs to definitions
    Then compilation succeeds
    And the resulting process contains the dangling target id

  Scenario: Missing frontmatter rejected
    Given an MDX input with no frontmatter
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Unknown type rejected
    Given an MDX input with unknown element type
    When I attempt to compile the MDX inputs to definitions
    Then compilation fails

  Scenario: Import rejects no process
    Given a BPMN definition with no process
    When I attempt to import the BPMN to MDX
    Then import fails

  Scenario: MDX compiles and imports back
    Given a minimal MDX input set with start, task, end, and two flows
    When I compile the MDX inputs to definitions
    Then importing the compiled definitions produces 5 MDX outputs
