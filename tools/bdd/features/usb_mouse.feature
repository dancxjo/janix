Feature: USB mouse input (xHCI HID)

  Background:
    Given the system boots on x86_64 in QEMU
    And the graph contains graph.devices

  Scenario: xHCI controller is discovered and exposed as MMIO bytespace
    Then the graph contains a Thing "device.usb.controller0" of kind "XhciController"
    And "device.usb.controller0" links to "bytespace.usb.xhci0.mmio"
    And mapping "bytespace.usb.xhci0.mmio" allows reading "HCIVERSION"

  Scenario: USB HID mouse produces PointerState updates
    Given the module "/boot/modules/usbd" is running
    And the graph contains "bytespace.mouse_input"
    When a USB mouse moves
    Then "bytespace.mouse_input" sequence increases
    And the graph contains a Thing "pointer.0" of kind "PointerState"
    And "pointer.0" position changes within 250 ms

  Scenario: Bloom consumes USB mouse input
    Given the module "/boot/modules/bloom" is running
    When a USB mouse moves
    Then the cursor sprite position changes on screen
