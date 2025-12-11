Graph Actualization Model

(ThingOS Hardware Delegation Architecture)

Overview

ThingOS does not expose hardware control through traditional syscalls, device files, or driver-specific APIs.
Instead, all hardware interaction is delegated through the system graph.

There are two flows:

Input Direction (Hardware → Kernel → Graph)
The kernel captures hardware events (via IRQs, controllers, timers) and records them into the graph as structured Things.

Output Direction (Userland → Graph → Kernel → Hardware)
Userland describes its intended hardware state by mutating Things in the graph, and the kernel actualizes those changes into real hardware effects.

The graph becomes the OS’s sole control plane.
The kernel becomes a graph actualizer.

Goals

No driver-specific syscalls.

Userland expresses intent through graph mutations.

Kernel handles physics by watching and actualizing those graph states.

Hardware behavior is always derived from graph state.

Uniform data model: everything is a Thing; actions are state changes.

Drives toward a declarative, introspectable, testable OS.

Input Pipeline

(Hardware → Kernel → Graph)

Hardware events are captured by kernel drivers:

Keyboard IRQs

Mouse motion + button packets

Serial input

Network frames

Timers

Sensors

The kernel does not interpret them further than necessary.
It simply records them as graph updates:

KeyEvent

MouseDelta or PointerEvent

SerialReceived

NetworkFrameReceived

TickEvent

SensorReading

Each is a Thing with appropriate fields (timestamp, device, data).

Userland processes then subscribe to or poll the graph to interpret these events.

Output Pipeline

(Userland → Graph → Kernel → Hardware)

Userland determines what hardware should do by mutating specific Things.

Examples:

Setting CursorState { x, y, visible }

Creating ScreenUpdate { region, buffer_ref, version }

Appending SerialWrite { port, data, state: Pending }

Setting SpeakerCommand { tone, duration }

Posting NetworkSendFrame { interface, bytes, state: Pending }

The kernel’s Actualizer Module scans the graph for these command or state Things:

Detects new or changed entries.

Performs the corresponding hardware action.

Updates result fields (state = Done, Error, or updated version).

The kernel does not decide what the user wants the hardware to do—
it merely actualizes the intent encoded in the graph.

Kernel Actualizer Module

A kernel subsystem (actualizer, graph_io, or similar) is responsible for observing the graph and applying userland intentions to hardware.

Responsibilities:

Periodically or reactively scan the graph.

Identify Things of interest (based on type/kind).

Detect changes via timestamps, versions, or state = Pending.

Perform the physical action:

Write to VRAM

Move hardware cursor registers

Send bytes to UART

Trigger speaker

Transmit network frames

Update the corresponding graph things with completion metadata.

This creates a stable and introspectable boundary:

Userland = declarative state

Kernel = physical actualization

Why This Architecture?
Declarative Hardware Control

Userland doesn’t issue imperative “do X” commands.
It declares the world as it should be.
Kernel ensures reality matches.

Perfect Traceability

Because all intent and all input are represented as Things, the entire history of OS behavior is auditably stored in the graph.

Simple Kernel ABI

Only two primitive actions required:

ThingCreate

ThingUpdate

Everything else is encoded in the graph schema.

Minimal Trusted Computing Base

Userland drivers remain unprivileged; they cannot touch hardware or sensitive kernel interfaces directly.

High Extensibility

New hardware types require no new syscalls—only new Thing kinds and an Actualizer handler.

Comparison to Traditional Models
Traditional OS	ThingOS
Syscalls, ioctls, device files	Graph mutations
Kernel drivers interpret semantics	Userland interprets semantics
Kernel enforces policy	Userland expresses intent; kernel actualizes
Userland calls kernel	Kernel watches userland state
Imperative commands	Declarative state description
Examples
Cursor Movement

Userland compositor writes:

CursorState {
    x: 640,
    y: 480,
    visible: true
}


Kernel actualizer:

Reads updated CursorState

Programs hardware cursor registers (or updates scanout)

Marks processed version

Serial Output

Userland writes:

SerialWrite {
    port: COM1,
    bytes: [0x48, 0x69], // "Hi"
    state: Pending
}


Kernel:

Finds Pending writes

Sends data through UART

Updates Thing to state = Done

Display Update

Userland writes:

A pixel buffer into shared memory

A Thing:

ScreenUpdate {
    region: {x, y, w, h},
    buffer_ref: ThingId,
    version: 42
}


Kernel:

Copies region into physical VRAM

Marks last-applied version

Future Layers

Once this foundation stabilizes:

“Unix” interfaces (like /dev or POSIX-ish APIs) can be projected over the graph model rather than baked into the kernel.

Wits (Quick, Combobulator, Memory) can treat hardware I/O as ordinary graph state.

Higher-level agents can reason about hardware the same way they reason about apps or windows.

Summary

In ThingOS:

Hardware → Graph expresses what happened.

Userland → Graph expresses what should happen.

Kernel Actualizer makes the physical world match the graph world.

This creates a coherent, declarative, introspective, and elegantly minimal OS model.
