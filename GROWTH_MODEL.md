# The ThingOS Growth Model

BRAN → Seed → Sprout → Bloom

ThingOS is not booted.
It is **grown**.

The system comes into existence through four distinct phases, each with a strictly limited role.
No phase may take responsibility that belongs to a later one.

If these boundaries blur, the system becomes fragile and opaque.

## BRAN

**Boot Runtime & Nucleation**

BRAN is the last imperative code that is allowed to be confused.

BRAN is the boot runner — the minimal runtime that exists only to bring the system into a state where the graph can exist.

### BRAN’s Responsibilities

BRAN is allowed to:
* Enter the system from firmware / bootloader
* Establish minimal CPU state
* Set up stack, paging, and early memory
* Bring up bare minimum I/O (e.g. serial for logging)
* Load initial binary artifacts (Seed, Sprout, assets)
* Transfer control cleanly and disappear

BRAN is not the kernel.
It is scaffolding.

### BRAN Must Not
* Create long-lived Things
* Encode policy
* Understand devices semantically
* Own drivers
* Persist state
* “Help” later stages

If BRAN knows why something exists, it already knows too much.

**BRAN’s success condition is self-erasure.**

## The Seed

**The Graph’s First Truth**

The Seed is the first moment the system can describe itself.

The Seed is pure data.

It is the initial graph artifact — schemas, initial Things, identities, and relationships that define what kind of system this instance is.

### What the Seed Contains

The Seed may include:
* Core schemas (Thing, Link, Bytespace, Device, Task, etc.)
* Initial namespaces
* Fundamental symbols / interned names
* Identity and version metadata
* Declared expectations (“there will be a framebuffer”)

The Seed does not execute.
It is read.

### What the Seed Must Never Contain
* Code
* Platform-specific assumptions
* Machine-specific details
* Runtime decisions

The Seed is portable across machines and platforms.

**If you can’t diff two Seeds meaningfully, you are hiding meaning in code.**

## The Sprout

**The First Living System**

The Sprout is the first Thing that knows it is a ThingOS.

Sprout is the initial running graph host.

This is the moment when:
* The graph becomes mutable
* Providers can attach
* The kernel asserts authority
* Isolation and capability begin

Sprout is typically the first kernel-hosted service.

### Sprout’s Responsibilities

Sprout:
* Instantiates the Seed into a live graph
* Brings up core graph providers
* Mounts machine descriptions
* Establishes scheduling
* Enables memory ownership
* Transitions from “boot” to “system”

Sprout is the minimum viable OS.

No UI.
No compositing.
No comfort.

Just coherence.

### Sprout Must Not
* Perform rendering
* Load user applications
* Decide UI policy
* Encode “desktop” assumptions

**Sprout’s job is to stabilize existence, not to impress.**

## Bloom

**The World Becomes Visible**

Bloom is where the system stops surviving and starts living.

Bloom is the first expressive phase of ThingOS.

This is where:
* The compositor comes online
* Windows exist
* Input becomes interactive
* Visual feedback appears
* Logs become visible Things
* The system can be experienced

Bloom is not required for correctness.
It is required for usefulness.

### Bloom’s Responsibilities

Bloom:
* Consumes graph state and renders it
* Visualizes relationships
* Hosts windows and surfaces
* Reflects system activity in real time
* Makes debugging experiential

Bloom does not own meaning.
It shows meaning.

### Bloom Must Not
* Contain boot logic
* Manage memory allocation
* Control hardware directly
* Become a policy god

Bloom is downstream of everything.
If Bloom crashes, the system should still exist.

## The Lifecycle Contract (Non-Negotiable)

| Phase | Knows About | Must Forget |
|:---|:---|:---|
| BRAN | How to start | Everything else |
| Seed | What exists | How it runs |
| Sprout | How to live | How to look |
| Bloom | How to appear | How to exist |

Violating this order creates:
* Hidden state
* Boot fragility
* “Just this once” hacks
* Irreversible coupling

## Why This Matters for Agents

Agents are especially dangerous at phase boundaries.

An agent will try to:
* Add convenience to BRAN
* Slip logic into the Seed
* Let Sprout do UI work
* Let Bloom make decisions

**Do not allow this.**

If a change touches two phases at once, it is almost always wrong.

## The Mental Model

Think of ThingOS like this:

1. **BRAN** lights the match
2. **Seed** defines the DNA
3. **Sprout** grows the organism
4. **Bloom** lets it be seen

Or, more bluntly:

**BRAN runs.**
**The Seed defines.**
**Sprout governs.**
**Bloom reveals.**

## Final Rule

If you are unsure where something belongs:

1. Ask when it is needed
2. Ask what it knows
3. Ask what would break if it vanished

Then place it in the earliest phase that does not require it.

That discipline is how ThingOS stays clean.
