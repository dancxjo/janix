import sys
import os

path = '/home/dancxjo/src/thing-os/tools/bdd/src/main.rs'
with open(path, 'r') as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if '.max_concurrent_scenarios(1)' in line:
        new_lines.append(line)
        new_lines.append('                .scenario_timeout(std::time::Duration::from_secs(600))\n')
    else:
        new_lines.append(line)

with open(path, 'w') as f:
    f.writelines(new_lines)
