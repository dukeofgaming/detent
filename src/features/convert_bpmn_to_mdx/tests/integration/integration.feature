Feature: Convert BPMN to MDX integration tests
  As a developer
  I want adapters to work with real BPMN and XSD artifacts
  So that file-based workflows are reliable

  Scenario: Hello-world imports to 6 MDX
    Given the hello-world BPMN fixture
    When I import it to MDX
    Then 6 MDX outputs are produced
    And each output contains a frontmatter block
    And each output contains its BPMN type

  Scenario: Hello-world BPMN metadata
    Given the hello-world BPMN fixture
    When I parse the BPMN to definitions
    Then the process id is "hello_world"
    And the process name is "hello-world"
    And the process is executable and public
    And the process documentation is "This is a hello world activity"
    And the definitions were exported by "jBPM Process Modeler" version "2.0"
    And the definitions target namespace is "http://www.omg.org/bpmn20"

  Scenario Outline: BPMN fixture imports to MDX
    Given the <fixture> BPMN fixture
    When I import it to MDX
    Then <count> MDX outputs are produced
    And each output contains a frontmatter block
    And each output contains its BPMN type

    Examples:
      | fixture      | count |
      | hello-world  | 6     |
      | blog-post    | 25    |
      | tdd          | 25    |

  Scenario: Parse preserves user and manual tasks
    Given BPMN XML preserving user and manual tasks
    When I parse the BPMN XML
    Then the process has one user task, one manual task, and one service task

  Scenario: Parse strips unsupported layout elements
    Given BPMN XML with an unsupported laneSet
    When I parse the BPMN XML
    Then both service tasks remain after stripping laneSet

  @xsd-validation
  Scenario Outline: Valid hello-world files pass XSD validation
    Given hello-world BPMN content from "<file>"
    When I validate the BPMN against XSD
    Then XSD validation succeeds

    Examples:
      | file               |
      | hello-world.bpmn   |
      | hello-world.bpmn2  |

  @xsd-validation
  Scenario Outline: Invalid element fails XSD validation
    Given hello-world BPMN content from "<file>"
    When I validate BPMN with invalid element in "<file>"
    Then XSD validation fails

    Examples:
      | file               |
      | hello-world.bpmn   |
      | hello-world.bpmn2  |

  @xsd-validation
  Scenario Outline: Missing required attribute fails XSD validation
    Given hello-world BPMN content from "<file>"
    When I validate BPMN missing targetNamespace in "<file>"
    Then XSD validation fails

    Examples:
      | file               |
      | hello-world.bpmn   |
      | hello-world.bpmn2  |
