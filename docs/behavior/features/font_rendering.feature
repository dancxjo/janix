@fonts @ui
Feature: Font Rendering and Typography

  As a system designer
  I want to verify that the OS can render text with different fonts and scripts
  So that I can ensure the typography subsystem is functioning correctly

  Scenario: Font Explorer displays multilingual text
    When I start the machine
    And I wait for the system to reach ready state
    Then I should see the Font Explorer window
