@smoke
Feature: Complete system boot smoke test
  As a kernel developer
  I want a single comprehensive boot verification
  So that I can quickly validate the entire boot sequence is working

  Scenario: Full boot sequence completes successfully
    Given the machine is started
    When I wait for the system to reach ready state
    
    # Kernel entry and early initialization
    Then the log should contain "thing-os kernel starting"
    And the log should match pattern "Memory map has \d+ entries"
    
    # Memory subsystem initialization
    And the log should match pattern "Frame allocator initialized with \d+ free frames"
    And the log should contain "Global allocator initialized"
    
    # Interrupt controller setup
    And the log should contain "IOAPIC: Init complete"
    
    # Scheduler and tasking ready
    And the log should contain "Scheduler initialized"
    
    # Root service and hardware discovery
    And the log should contain "Spawning Root service"
    And the log should contain "ROOT: boot registration begin"
    And the log should contain "PCI: Starting enumeration"
    And the log should match pattern "ROOT: registered items. host=\d+ kernel=\d+"
    
    # Graph census verification
    And the log should contain "ROOT DUMP NODES"
    And the log should contain "dev.Host"
    And the log should contain "dev.bus.Pci"
    And the log should contain "proc.Kernel"
    And the log should contain "ROOT DUMP EDGES"
    And the log should contain "[:HAS_BUS]"
    
    # Census complete and system ready
    And the log should match pattern "KERNEL: root census complete: host=t\d+ kernel=t\d+ root=t\d+"
    
    # Sprout supervisor launching
    And the log should contain "Found sprout module, loading"
    And the log should contain "Spawning sprout"
    And the log should contain "Entering user mode"
    
    # Sprout starting and discovery
    And the log should match pattern "SPROUT: v\d+\.\d+ starting"
    And the log should contain "SPROUT: devtree::init entry"
    And the log should contain "SPROUT: Supervisor starting"
    And the log should match pattern "SPROUT: Found \d+ modules"
    
    # Root self-test passing
    And the log should contain "ROOT SELFTEST: PASS"
    
    # Main scheduler loop entered
    And the log should contain "System initialized. Entering scheduler loop"
    
    # Applications launched
    And the log should match pattern "SPROUT: App launched \(PID=\d+\)"
