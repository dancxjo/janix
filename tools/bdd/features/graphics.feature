Feature: Framebuffer surface is a first-class thing

@fb @core
Scenario Outline: Framebuffer device is published as Device/Surface/Bytespace
  Given I boot ThingOS on "<arch>" with a framebuffer
  When the kernel enumerates display
  Then the graph contains device.display.0 with Kind kind.Device
  And it links to surface.display.0 with Kind kind.Surface
  And surface.display.0 links to a bytespace.fb.0 describing the pixel memory
  And the surface records width/height/format/stride

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@fb @core
Scenario Outline: Early logging can draw to framebuffer without userspace
  Given I boot ThingOS on "<arch>" with a framebuffer
  When the kernel logs "EARLY" before userspace is running
  Then serial contains "EARLY"
  And the framebuffer contains visible evidence of the log

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@fb @core
Scenario Outline: A user app draws a rectangle and it appears on the primary surface
  Given I boot ThingOS on "<arch>" with framebuffer
  And apps/clock starts as a user task
  When it requests a drawing surface
  And it draws a filled rectangle at (10,10) size (80x40)
  Then the framebuffer checksum changes from the boot checksum
  And the graph contains draw.rect events linked to task.clock

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@fb @core
Scenario Outline: Userspace drawing is mediated, not raw framebuffer writes
  Given I boot ThingOS on "<arch>" with framebuffer
  When a userspace task attempts to write directly into framebuffer physical memory
  Then it faults or is prevented
  And drawing must occur via the surface contract

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
