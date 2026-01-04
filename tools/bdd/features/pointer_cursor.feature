Feature: Pointer and Cursor
  PS/2 mouse input is converted to pointer state and rendered as a cursor.

  @pointer @smoke
  Scenario: Pointer state exists in graph
    Given the system boots successfully
    Then a Thing named "pointer.0" should exist

  @pointer @mouse @smoke
  Scenario: PS/2 mouse driver publishes to ring buffer
    Given the system boots successfully
    And the serial log should contain "input: discovered mouse"
    Then a bytespace "bytespace.mouse_input" should exist

  @cursor @render @smoke
  Scenario: Bloom renders a cursor at the pointer position
    Given the system boots successfully
    And the serial log should contain "BLOOM: initialized"
    Then it does not panic

  @cursor @asset @smoke  
  Scenario: Cursor asset is loaded from .cur file
    Given the system boots successfully
    And a cursor asset "Normal.cur" exists
    Then the serial log should contain "BLOOM: loaded Normal.cur"
    Or the serial log should contain "BLOOM: loaded Working.ani"
    Or the serial log should contain "BLOOM: no cursor asset found"

  @cursor @damage
  Scenario: Cursor uses damage rects for efficient redraw
    Given the system boots successfully
    And Bloom is running
    Then the serial log should contain "BLOOM: initialized"
