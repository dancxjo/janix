@multitasking @visual
Feature: Visual Multitasking

  As a user
  I want to see multiple applications running simultaneously
  So that I can monitor different information streams simultaneously

  Scenario: Clock and Font Explorer are visible together
    Given the machine is booted
    Then I should see the "Clock" window
    And I should see the "Font Explorer" window
