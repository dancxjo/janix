---
name: "Wayland Terminal: Terminal Backend (PTY)"
about: "Implement PTY/fake loop backend for terminal."
title: "Wayland Terminal: Terminal Backend (PTY/Fake Loop)"
labels: ["wayland","terminal","janix","subtask"]
---

## Task
- Implement PTY allocation and shell spawn
- Wire up I/O loop between Wayland client and PTY
- Provide fallback/fake loop if PTY not available

## Acceptance Criteria
- [ ] PTY-backed shell works in terminal
- [ ] I/O loop functional
- [ ] Fallback/fake loop documented
- [ ] Linked to main epic

---
**Parent Epic:** See main milestone issue.
