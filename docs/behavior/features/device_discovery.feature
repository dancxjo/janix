@devd @wip
Feature: Device Discovery and Driver Lifecycle (devd)

  As a system developer
  I want devd to automatically detect hardware and launch the appropriate driver
  So that devices are available without any hardcoded driver spawning in init

  @wip
  Scenario: devd launches the VirtIO-net driver automatically on boot
    Given the system has booted
    When sufficient time has elapsed for devd to scan devices
    Then the path /dev/net/virtio0/mac should be readable
    And the output should match a MAC address pattern

  @wip
  Scenario: devd restarts the VirtIO-net driver after a crash
    Given the system has booted
    And devd has launched virtio_netd for the virtio-net device
    When the virtio_netd process exits unexpectedly
    Then devd should relaunch virtio_netd within a reasonable backoff period
    And the path /dev/net/virtio0/mac should be readable again

  @wip
  Scenario: /sys/devices/ exposes PCI devices discovered by the kernel
    Given the system has booted
    When I read the directory /sys/devices/
    Then the output should contain at least one entry matching "pci-"
    And each device entry should contain a readable "handle" file
    And the contents of each "handle" file should be a valid decimal number
