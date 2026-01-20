@input @mouse
Feature: Pointer Input

  As a user
  I want mouse input to be received and reflected visually
  So that I can interact with the system using a pointing device

  Scenario: Cursor responds to mouse movement
    Given a cursor is visible on the screen
    When I move the mouse
    Then the cursor should move correspondingly on the screen
