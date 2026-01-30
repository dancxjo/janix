---
trigger: always_on
---

The command you should use the most (to see if things compile, etc.) is `just run`. Always run `just run` one last time before submitting your work to make sure it just runs. *But* this command will run until the system faults in qemu, so expect it to run permanently. Trying running with a timeout.