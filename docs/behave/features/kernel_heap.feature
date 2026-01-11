Feature: Kernel Heap

  Scenario: Heap Initialization and Basic Allocation
    Given the machine is booting
    Then the log should contain "Initializing Kernel Heap..."
    And the log should contain "kheap: grew by 64 pages"
    And the log should contain "global_alloc: switched to kernel heap"
    And the log should contain "kheap: sanity ok"

  Scenario: Heap Growth on Demand
    Given the machine is booting
    Then the log should contain "kheap: forcing growth..."
    # Initial growth was 64 pages.
    # We allocate 300KB. Initial was 256KB.
    # It should trigger another growth.
    # The growth increment is KHEAP_GROW_PAGES = 16 pages = 64KB.
    # It might grow multiple times or just once depending on alloc request size vs chunk.
    # We just check that it grew again.
    And the log should contain "kheap: grew by 16 pages"
    And the log should contain "kheap: big allocation ok"
    
  Scenario: Diagnostics
    Given the machine is booting
    Then the log should contain "kheap: reserved="
    And the log should contain "committed="
