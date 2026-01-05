Feature: hello_window renders a widget layout as Things with a shared drop shadow

  Background:
    Given the system boots to Sprout
    And Bloom is running as a userspace compositor
    And Bloom owns a Bytespace-backed framebuffer surface

  Scenario: hello_window publishes a Window Thing with a widget tree and Bloom renders it
    Given an app "hello_window" is started as a user task
    When "hello_window" creates a Window Thing sized 480x272 titled "Hello"
    And "hello_window" creates a Layout Thing of kind "Column" with padding 12 and gap 10
    And "hello_window" creates a Label Thing with text "Hello, ThingOS."
    And "hello_window" creates a Button Thing with text "OK"
    And "hello_window" links the Layout as the Window content root
    And "hello_window" links the Label and Button as children of the Layout in order
    And "hello_window" sets style on the Window:
      | background | rgba(245,245,245,255) |
      | radius     | 10                    |
      | shadow     | drop                  |
      | elevation  | 2                     |
    And "hello_window" commits a new Frame Thing for the Window
    Then Bloom must observe the Window Thing and its widget tree
    And Bloom must render the Window rectangle with rounded corners
    And Bloom must render a drop shadow behind the Window using the shared shadow kernel
    And Bloom must render the Label and Button positioned by the Layout
    And the framebuffer must contain non-background pixels in the Window region

  Scenario: drop shadow implementation is shared with cursor shadow
    Given Bloom has cursor rendering enabled
    When Bloom renders the cursor and renders a shadowed Window in the same frame
    Then both shadows must use the same shadow kernel implementation
    And the kernel must accept two call sites:
      | callsite | params                           |
      | cursor   | sprite bounds + hotspot           |
      | window   | rounded rect bounds + radius      |
