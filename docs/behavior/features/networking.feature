@network @wip
Feature: Network VFS Provider (/net/)

  As a system developer
  I want to verify that netd exposes the network stack via the /net/ VFS tree
  So that applications can interact with the network using ordinary file operations

  @wip
  Scenario: Interface status is readable via /net/
    Given the machine is running
    When I wait for netd to mount /net/
    Then reading /net/interfaces/eth0/status should return output containing "state:"

  @wip
  Scenario: Allocating a TCP socket via /net/tcp/new
    Given the machine is running
    When I wait for netd to mount /net/
    Then reading /net/tcp/new should return a fresh socket ID

  @wip
  Scenario: Routing table is readable via /net/routes
    Given the machine is running
    When I wait for netd to mount /net/
    Then reading /net/routes should return non-empty output
