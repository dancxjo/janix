@gallery @assets
Feature: Asset Gallery

  As a user
  I want to browse available SVG assets in a visual grid
  So that I can see what icons are installed on the system

  Scenario: User browses Photosynthesis gallery
    When I start the machine
    Then I should see a window with title "Photosynthesis (SVG Grid)"
    And I should see at least 6 asset tiles
    And I should see the "meta.graph.svg" icon in the grid
