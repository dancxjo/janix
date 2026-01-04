# BDD Test Report Index

## Feature: Boot contract and system bring-up

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| Bran hands off a boot contract to the Kernel | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/boot_contract_and_system_bring_up/bran_hands_off_a_boot_contract_to_the_kernel/report.md) |
| Sprout starts and publishes its presence in the graph | ❌ | ❌ | ❌ | ❌ | [View Report](x86_64/boot_contract_and_system_bring_up/sprout_starts_and_publishes_its_presence_in_the_graph/report.md) |
| Sprout starts core services | ❌ | ❌ | ❌ | ❌ | [View Report](x86_64/boot_contract_and_system_bring_up/sprout_starts_core_services/report.md) |
| The Kernel exposes a root Place and a devices Place | ❌ | ❌ | ❌ | ❌ | [View Report](x86_64/boot_contract_and_system_bring_up/the_kernel_exposes_a_root_place_and_a_devices_place/report.md) |

## Feature: Bytespaces and address spaces

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A process can create a bytespace and map it with permissions | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/bytespaces_and_address_spaces/a_process_can_create_a_bytespace_and_map_it_with_permissions/report.md) |
| Task stacks have explicit ownership and do not leak | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/bytespaces_and_address_spaces/task_stacks_have_explicit_ownership_and_do_not_leak/report.md) |
| Unmapping removes access | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/bytespaces_and_address_spaces/unmapping_removes_access/report.md) |

## Feature: Capabilities and least privilege

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A process cannot map memory without a mapping capability | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/capabilities_and_least_privilege/a_process_cannot_map_memory_without_a_mapping_capability/report.md) |
| A process cannot read raw input without an input capability | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/capabilities_and_least_privilege/a_process_cannot_read_raw_input_without_an_input_capability/report.md) |
| Framebuffer details are not exposed without explicit capability | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/capabilities_and_least_privilege/framebuffer_details_are_not_exposed_without_explicit_capability/report.md) |
| Insecure escape hatches are feature-flagged | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/capabilities_and_least_privilege/insecure_escape_hatches_are_feature_flagged/report.md) |

## Feature: Demo app vertical slice

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| logview draws and receives input | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/demo_app_vertical_slice/logview_draws_and_receives_input/report.md) |
| logview exits without tearing down the world | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/demo_app_vertical_slice/logview_exits_without_tearing_down_the_world/report.md) |
| logview logs through syscall and appears in the graph | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/demo_app_vertical_slice/logview_logs_through_syscall_and_appears_in_the_graph/report.md) |
| logview starts as a user task with a heap | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/demo_app_vertical_slice/logview_starts_as_a_user_task_with_a_heap/report.md) |

## Feature: Graph REPL over Serial (OpenGQL)

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| Create a Thing using CREATE | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/create_a_thing_using_create/report.md) |
| Create a node | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/create_a_node/report.md) |
| Create a relationship | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/create_a_relationship/report.md) |
| Create a relationship between Things | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/create_a_relationship_between_things/report.md) |
| Empty input does nothing | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/empty_input_does_nothing/report.md) |
| Empty input is ignored | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/empty_input_is_ignored/report.md) |
| Exit the REPL | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/exit_the_repl/report.md) |
| Invalid OpenGQL syntax | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/invalid_opengql_syntax/report.md) |
| Multi-line OpenGQL query | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/multi_line_opengql_query/report.md) |
| Multi-line query input | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/multi_line_query_input/report.md) |
| Query history is preserved | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/query_history_is_preserved/report.md) |
| Query history is recorded | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/query_history_is_recorded/report.md) |
| Query nodes using MATCH | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/query_nodes_using_match/report.md) |
| Query returns no results | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/query_returns_no_results/report.md) |
| Query with no matching results | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/query_with_no_matching_results/report.md) |
| REPL announces readiness | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/repl_announces_readiness/report.md) |
| REPL exit command | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/repl_exit_command/report.md) |
| Semantic error handling | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/semantic_error_handling/report.md) |
| Simple MATCH query | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/simple_match_query/report.md) |
| Syntax error handling | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/syntax_error_handling/report.md) |
| Unknown REPL command | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/unknown_repl_command/report.md) |
| Unknown command | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/unknown_command/report.md) |
| Valid syntax but invalid semantics | ✅ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/graph_repl_over_serial__opengql_/valid_syntax_but_invalid_semantics/report.md) |

## Feature: Graph as the system

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A task is linked to its owning process | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/graph_as_the_system/a_task_is_linked_to_its_owning_process/report.md) |
| Devices appear in place.devices | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/graph_as_the_system/devices_appear_in_place_devices/report.md) |
| Tasks are represented as Things | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/graph_as_the_system/tasks_are_represented_as_things/report.md) |
| The graph is queryable at runtime | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/graph_as_the_system/the_graph_is_queryable_at_runtime/report.md) |

## Feature: Input pipeline and focus model

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| Focus determines which surface receives TextEvent | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/input_pipeline_and_focus_model/focus_determines_which_surface_receives_textevent/report.md) |
| Raw scancodes can be read by inputd with capability | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/input_pipeline_and_focus_model/raw_scancodes_can_be_read_by_inputd_with_capability/report.md) |
| inputd publishes KeyEvent Things to the graph | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/input_pipeline_and_focus_model/inputd_publishes_keyevent_things_to_the_graph/report.md) |

## Feature: Keyboard input pipeline

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A layout service can convert key events to text events | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/a_layout_service_can_convert_key_events_to_text_events/report.md) |
| Changing focus changes routing immediately | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/changing_focus_changes_routing_immediately/report.md) |
| Delivery can be traced | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/delivery_can_be_traced/report.md) |
| Delivery is non-blocking | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/delivery_is_non_blocking/report.md) |
| Focus determines which window receives keyboard events | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/focus_determines_which_window_receives_keyboard_events/report.md) |
| Key activity produces raw key events | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/key_activity_produces_raw_key_events/report.md) |
| Key events are deliverable to user programs | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/key_events_are_deliverable_to_user_programs/report.md) |
| Keyboard events are buffered during bursts | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/keyboard_events_are_buffered_during_bursts/report.md) |
| Keyboard events are observable as Things | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/keyboard_events_are_observable_as_things/report.md) |
| Multiple consumers can observe raw key events | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/multiple_consumers_can_observe_raw_key_events/report.md) |
| No heavy work is required to capture key events | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/no_heavy_work_is_required_to_capture_key_events/report.md) |
| Non-printing keys do not produce text | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/non_printing_keys_do_not_produce_text/report.md) |
| The system exposes a keyboard-capable input device when available | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/the_system_exposes_a_keyboard_capable_input_device_when_available/report.md) |
| The system still boots without a keyboard | ❌ | ⚪ | ⚪ | ⚪ | [View Report](x86_64/keyboard_input_pipeline/the_system_still_boots_without_a_keyboard/report.md) |

## Feature: Multi-architecture behavioral parity

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| Boot reaches kernel ready on each architecture | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/multi_architecture_behavioral_parity/boot_reaches_kernel_ready_on_each_architecture/report.md) |
| Missing devices are represented as absence, not failure | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/multi_architecture_behavioral_parity/missing_devices_are_represented_as_absence__not_failure/report.md) |
| The syscall ABI returns a structured result on each architecture | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/multi_architecture_behavioral_parity/the_syscall_abi_returns_a_structured_result_on_each_architecture/report.md) |

## Feature: Scheduler liveness and fairness

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A blocked task wakes when its watch fires | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/scheduler_liveness_and_fairness/a_blocked_task_wakes_when_its_watch_fires/report.md) |
| A task may block on a deadline watch | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/scheduler_liveness_and_fairness/a_task_may_block_on_a_deadline_watch/report.md) |
| Runnable tasks eventually run | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/scheduler_liveness_and_fairness/runnable_tasks_eventually_run/report.md) |
| The scheduler switches tasks over time | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/scheduler_liveness_and_fairness/the_scheduler_switches_tasks_over_time/report.md) |

## Feature: Surfaces and compositing

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A surface is a Thing with ownership and bounds | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/surfaces_and_compositing/a_surface_is_a_thing_with_ownership_and_bounds/report.md) |
| Bloom can paint the screen a solid color | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/surfaces_and_compositing/bloom_can_paint_the_screen_a_solid_color/report.md) |
| The display backend is swappable | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/surfaces_and_compositing/the_display_backend_is_swappable/report.md) |

## Feature: Swappable display backends

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| Bloom can paint a solid color on any provider | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/swappable_display_backends/bloom_can_paint_a_solid_color_on_any_provider/report.md) |
| Bloom cannot access display bytespace without capability | ✅ | ❌ | ❌ | ✅ | [View Report](x86_64/swappable_display_backends/bloom_cannot_access_display_bytespace_without_capability/report.md) |
| Limine framebuffer provider exposes a primary display bytespace | ✅ | ❌ | ❌ | ❌ | [View Report](x86_64/swappable_display_backends/limine_framebuffer_provider_exposes_a_primary_display_bytespace/report.md) |
| RAMFB provider exposes the same graph contract | ✅ | ✅ | ✅ | ❌ | [View Report](x86_64/swappable_display_backends/ramfb_provider_exposes_the_same_graph_contract/report.md) |

## Feature: Watches and event-driven waiting

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |
| --- | :---: | :---: | :---: | :---: | :---: |
| A device watch fires on an input event | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/watches_and_event_driven_waiting/a_device_watch_fires_on_an_input_event/report.md) |
| A graph watch fires when a Thing is added to a Place | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/watches_and_event_driven_waiting/a_graph_watch_fires_when_a_thing_is_added_to_a_place/report.md) |
| Waiting on "any watch" returns when one fires | ✅ | ✅ | ✅ | ✅ | [View Report](x86_64/watches_and_event_driven_waiting/waiting_on__any_watch__returns_when_one_fires/report.md) |

