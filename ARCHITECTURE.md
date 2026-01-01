# ThingOS Architectural Manifesto

How the system is actually shaped

## The Three Structural Axes of ThingOS

Every architectural decision in ThingOS exists at the intersection of three orthogonal axes:

* **What something is** → Thing / Schema
* **Where it runs** → Platform
* **How it touches reality** → Machine

Confusing these axes is the single most common architectural error.
Agents must keep them strictly separated.

## Platforms: Where Code Runs

A **Platform** describes the execution environment for code.

Examples:
* x86_64
* aarch64
* riscv64
* loongarch64

A platform answers questions like:
* Instruction set?
* Calling convention?
* Trap / exception model?
* Page table format?
* Atomic guarantees?

A platform does not describe hardware devices.
It describes how code executes.

### Platform Rules

* Platform-specific code lives only where execution semantics differ
* Platforms must not contain drivers
* Platforms must not encode policy
* Platforms are allowed to be boring

If code exists solely because this CPU works differently, it belongs to the platform.

## Machines: How the World Is Touched

A **Machine** is the concrete interface between software and physical reality.

Machines describe:
* Interrupt controllers
* Timers
* Serial ports
* Framebuffers
* Memory maps
* Power states

A machine answers:
**“How does this system physically behave?”**

Examples:
* QEMU virt machine
* Laptop motherboard
* SBC with custom peripherals

A machine is not an ISA.
A machine is not a driver.
A machine is a bundle of hardware facts.

### Machine Rules

* Machines expose capabilities, not behavior
* Machines are queried, not assumed
* Machines do not schedule, allocate, or decide
* Machines do not interpret meaning

A machine says what exists, never what to do.

## Drivers: Adapters, Not Authorities

Drivers are Things that adapt machine capabilities into usable graph interfaces.

Drivers:
* Translate hardware registers into structured state
* Convert interrupts into events
* Expose Bytespaces, Streams, or Signals

Drivers do not:
* Own global policy
* Decide scheduling
* Control lifecycle beyond their scope
* Hide state

A driver is a lens, not a ruler.

### Where Drivers Live

Drivers live outside the platform and outside the kernel core.

They are:
* Graph providers
* Loadable services
* Replaceable components

If a driver requires a different ISA, that is a platform concern.
If it requires different registers, that is a machine concern.
The driver itself must sit between, not inside either.

## The Kernel: A Graph Host, Not a Boss

The kernel is a trusted graph provider.

It:
* Boots the system
* Mounts initial graphs
* Enforces isolation
* Arbitrates access
* Hosts early providers

It does not:
* Define meaning
* Encode workflows
* Own devices conceptually
* Act as a god-object

The kernel’s job is to make graphs possible, not opinionated.

If logic feels “high-level,” it probably does not belong in the kernel.

## Graph Providers: Where Things Come From

A **Graph Provider** is any component that introduces Things into the system graph.

Examples:
* The kernel itself
* A driver
* A scheduler
* A compositor
* A user-space service

Graph providers:
* Declare schemas
* Instantiate Things
* Maintain links
* Publish events

They do not:
* Assume exclusivity
* Hide internal state
* Mutate unrelated graphs

Multiple providers may coexist.
Conflict is resolved by links and queries, not authority.

## Scheduling Is a Graph Problem

Schedulers are not hard-coded subsystems.
They are graph providers that introduce:
* Task Things
* Run queues
* Time slices
* State transitions

Scheduling policy is:
* Inspectable
* Replaceable
* Queryable

If a task is running, you must be able to ask why.

## Memory Is Not Global

There is no “the heap.”

There are:
* Bytespaces
* Ownership links
* Sharing relationships
* Lifetime constraints

Memory allocation is a relationship, not an event.

If memory appears without an owner, that is a design failure.

## User Space Is Not Second-Class

User programs are graph participants.

They:
* Create Things
* Query state
* Subscribe to events
* Provide services

They are constrained by:
* Capability links
* Namespaces
* Isolation rules

Not by:
* Arbitrary syscall walls
* Fake abstractions
* Legacy process models

## Naming Is Architecture

Names are semantic commitments.

If something is called:
* **machine** — it must be about physical reality
* **platform** — it must be about execution
* **driver** — it must adapt, not decide
* **kernel** — it must host, not rule

Renaming is not cosmetic.
Renaming is architectural correction.

## Where Things Belong (Non-Negotiable)

| Concern | Lives In |
|:---|:---|
| ISA differences | Platform |
| Hardware layout | Machine |
| Hardware adaptation | Driver |
| Policy | Graph provider |
| Execution authority | Kernel |
| Meaning | Schema |
| State | Graph |

If a change crosses these boundaries, stop and reconsider.

## The Agent’s Prime Directive (Revised)

When working on ThingOS, always ask:

1. What Thing is this?
2. What schema defines it?
3. Which graph provider owns it?
4. Does this belong to platform, machine, or neither?
5. Can this be replaced without collapse?

If you cannot answer all five, do not write code yet.

## Final Architectural Truth

ThingOS is not layered like a cake.
It is interlinked like a map.

Authority comes from structure.
Clarity comes from explicitness.
Power comes from composition.

When in doubt:

**Move meaning into the graph and remove it from the code.**

That is how ThingOS stays alive.
