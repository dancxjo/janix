Feature: Input pipeline and focus

@core
Scenario Outline: Raw scancodes flow kernel -> userspace inputd -> graph KeyEvent
  Given I boot ThingOS on "<arch>"
  And the keyboard device is present
  And inputd is running
  When I inject a key press "A"
  Then the kernel receives a scancode into a ring buffer
  And inputd reads scancodes via sys_input_read
  And the graph contains a KeyEvent node with key=A state=Pressed
  And a TextEvent node with text="a" is published

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Focus determines where text events go
  Given I boot ThingOS on "<arch>"
  And there are two windows window.1 and window.2
  And place.input --[focused]--> window.2
  When I type "hi"
  Then the graph publishes TextEvent nodes linked to window.2
  And no TextEvent nodes are linked to window.1

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
