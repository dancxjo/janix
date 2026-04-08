---
name: "Milestone: Port Minimal Wayland Terminal Emulator to Janix"
about: "Epic and sub-issues for porting a minimal Wayland terminal emulator to Janix."
title: "Milestone: Port Minimal Wayland Terminal Emulator to Janix"
labels: ["epic", "wayland", "terminal", "janix"]
assignees: []
---

# Milestone: Port Minimal Wayland Terminal Emulator to Janix

This epic tracks the porting and integration of a minimal Wayland terminal emulator inside a Janix session. All sub-issues should be linked here for traceability.

## Goal
- Port and run a minimal Wayland terminal emulator as a Janix userland app.
- Integrate with Janix session/compositor stack.
- Achieve basic text rendering, keyboard input, and PTY-backed shell.

## Sub-Issues
- [ ] Select target terminal emulator (foot, st, tinywl-term, or minimal custom)
- [ ] Audit and port dependencies (Wayland client, font/text, PTY, VFS)
- [ ] Cross-compile and build for Janix (no_std, stem, platform PAL)
- [ ] Bring up Wayland client connection to Janix compositor
- [ ] Implement text rendering (font raster, glyph cache, draw to surface)
- [ ] Keyboard input handling (Wayland key events → PTY)
- [ ] Terminal backend: PTY allocation, shell spawn, I/O loop
- [ ] Integrate with Janix session management (launch, focus, close)
- [ ] Document porting notes, constraints, and future work

## Constraints
- Must use `#![no_std]` and depend only on `core`, `alloc`, and `stem` (see AGENTS.md)
- All platform primitives via `stem::pal`
- No direct use of Rust `std` or OS-specific APIs
- All dependencies must be audited for platform boundary compliance
- Prefer minimal, auditable code over feature completeness

## Deliverables
- Working terminal emulator running as a Wayland client in Janix
- Source code and build instructions
- Documentation of porting process, constraints, and gaps
- Linked sub-issues for each milestone step

---
**Parent Epic:** This issue is the parent for all sub-issues related to the Wayland terminal porting milestone. Link sub-issues below.

/cc @dancxjo
