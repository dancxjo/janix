@rendering @apps
Feature: Custom Vector Rendering

  As a user
  I want applications to be able to render custom vector graphics
  So that I can use visual tools beyond simple text

  Scenario: Application renders custom graphics
    Given the machine is running
    And I wait for 5 seconds
    Then I should see a window at 100, 100 with background color "#F5F5F0"
    And I should see a rectangle at 120, 120 with size 120x80 and color "#22AA66"
    When I wait for 2 seconds
    Then I should see a rectangle at 120, 120 with size 120x80 and color "#AA2244"
