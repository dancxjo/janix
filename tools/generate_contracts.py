#!/usr/bin/env python3
import os
import re
import sys
import glob
import subprocess

def parse_brace_block(text, start_index):
    """Parses a code block enclosed in braces, handling nesting."""
    if start_index == -1: return ""
    brace_count = 0
    i = start_index
    started = False
    while i < len(text):
        if text[i] == '{':
            if not started:
                started = True
            brace_count += 1
        elif text[i] == '}':
            brace_count -= 1
            if started and brace_count == 0:
                return text[start_index:i+1]
        i += 1
    return ""

def clean_type(t):
    return t.strip().replace('\n', ' ').replace('  ', ' ')

def generate_syscalls():
    print("Generating docs/contracts/syscalls.md...")

    syscalls_map = {}
    with open('abi/src/syscalls.rs', 'r') as f:
        for line in f:
            m = re.search(r'pub const (SYSCALL_\w+): u64 = (\d+);', line)
            if m:
                syscalls_map[m.group(1)] = int(m.group(2))

    arch_path = 'arch/src/x86_64/syscall.rs'
    if not os.path.exists(arch_path):
        match_block = ""
    else:
        with open(arch_path, 'r') as f:
            arch_content = f.read()

        match_start = arch_content.find('match num {')
        if match_start != -1:
            match_block = parse_brace_block(arch_content, match_start)
        else:
            match_block = ""

    handlers = {}

    for name in syscalls_map.keys():
        arm_regex = re.compile(rf'\b{name}\s*=>')
        m = arm_regex.search(match_block)

        if not m:
            handlers[name] = {'req': 'N/A', 'handler': 'Not found in x86_64/syscall.rs', 'notes': 'Missing handler'}
            continue

        remainder = match_block[m.end():]
        remainder = remainder.lstrip()

        body = ""
        if remainder.startswith('{'):
            body = parse_brace_block(remainder, 0)
        else:
            comma = remainder.find(',')
            if comma != -1:
                body = remainder[:comma]
            else:
                body = remainder

        req_variant = "N/A"
        handler_path = "Inline/Unknown"
        notes = []

        req_match = re.search(r'KernelRequest::(\w+)', body)
        if req_match:
            req_variant = req_match.group(1)
            handler_path = f"kernel::handle_request ({req_variant})"

        if "handle_request" in body and req_variant == "N/A":
             handler_path = "kernel::handle_request (Dynamic/Complex)"

        if req_variant == "N/A":
            if "yield_current_thread" in body: handler_path = "kernel::sched::yield_current_thread"
            elif "exit_current_thread" in body: handler_path = "kernel::sched::exit_current_thread"
            elif "sys_sleep_for_ns" in body: handler_path = "crate::user::sys_sleep_for_ns"
            elif "create_thing" in body: handler_path = "kernel::graph::create_thing"
            elif "update_thing" in body: handler_path = "kernel::graph::update_thing"
            elif "add_link" in body: handler_path = "kernel::graph::add_link"
            elif "link_target_at" in body: handler_path = "kernel::graph::link_target_at"
            elif "register_shared_buffer" in body: handler_path = "kernel::shared_buffer::register_shared_buffer"
            elif "dev_open" in body: handler_path = "kernel::bridge::ps2::dev_open"
            elif "dev_read" in body: handler_path = "kernel::bridge::ps2::dev_read"
            elif "sys_resident_alloc" in body: handler_path = "kernel::resident::manager::sys_resident_alloc"
            elif "sys_resident_map" in body: handler_path = "kernel::resident::manager::sys_resident_map"
            elif "sys_resident_unmap" in body: handler_path = "kernel::resident::manager::sys_resident_unmap"
            elif "sys_thing_rest" in body: handler_path = "kernel::resident::manager::sys_thing_rest"
            elif "symbols::intern" in body: handler_path = "kernel::symbols::intern"
            elif "symbols::resolve" in body: handler_path = "kernel::symbols::resolve"

        if "unsafe" in body: notes.append("unsafe block")
        if "user_slice" in body or "user_ptr" in body: notes.append("direct user memory")
        if "transmute" in body: notes.append("transmute")
        if "TryFrom" in body: notes.append("TryFrom validation")

        handlers[name] = {'req': req_variant, 'handler': handler_path, 'notes': ", ".join(notes)}

    sorted_syscalls = sorted(syscalls_map.items(), key=lambda x: x[1])

    with open('docs/contracts/syscalls.md', 'w') as f:
        f.write("# Syscall Dispatch Map\n\n")
        f.write("| Number | Syscall Name | KernelRequest Variant | Kernel Handler | Notes |\n")
        f.write("|---|---|---|---|---|\n")
        for name, num in sorted_syscalls:
            info = handlers.get(name, {'req': '?', 'handler': '?', 'notes': '?'})
            f.write(f"| {num} | `{name}` | `{info['req']}` | `{info['handler']}` | {info['notes']} |\n")

def check_forbidden(type_str):
    forbidden = []
    if "&str" in type_str: forbidden.append("&str")
    if "&'static str" in type_str: forbidden.append("&'static str")
    if "&[" in type_str: forbidden.append("&[T]")
    if "Vec<" in type_str: forbidden.append("Vec<T>")
    if re.search(r'<\w+>', type_str) and "UserSlice" not in type_str and "UserPtr" not in type_str and "Option" not in type_str and "PhantomData" not in type_str and "SysRet" not in type_str:
         forbidden.append("Generics")
    return forbidden

def parse_enums_structs(content, file_path):
    items = []
    iterator = re.finditer(r'pub (enum|struct) (\w+)', content)
    for match in iterator:
        name = match.group(2)
        kind = match.group(1)
        start = match.start()

        block_start = content.find('{', start)
        if block_start == -1: continue

        block = parse_brace_block(content, block_start)

        lines = block.split('\n')
        for line in lines:
            line = line.strip()
            if not line or line.startswith('pub enum') or line.startswith('pub struct') or line.startswith('{') or line == '}': continue
            if line.startswith('//'): continue

            line = re.sub(r'//.*', '', line)

            if ':' in line:
                parts = line.split(':')
                fname = parts[0].strip()
                ftype = parts[1].strip().rstrip(',')
                forbidden = check_forbidden(ftype)
                items.append({'kind': kind, 'name': name, 'field': fname, 'type': ftype, 'forbidden': forbidden, 'file': file_path})
            elif '(' in line and ')' in line:
                 vname = line.split('(')[0].strip()
                 types = line.split('(')[1].split(')')[0]
                 for t in types.split(','):
                     t = t.strip()
                     if t:
                         forbidden = check_forbidden(t)
                         items.append({'kind': kind, 'name': name, 'variant': vname, 'type': t, 'forbidden': forbidden, 'file': file_path})
            elif ',' in line or (not '{' in line and not '}' in line):
                 vname = line.strip().rstrip(',')
                 items.append({'kind': kind, 'name': name, 'variant': vname, 'type': 'Unit', 'forbidden': [], 'file': file_path})

    return items

def generate_abi_surface():
    print("Generating docs/contracts/abi_surface.md...")

    with open('docs/contracts/abi_surface.md', 'w') as f:
        f.write("# ABI Surface Inventory\n\n")

        with open('abi/src/lib.rs', 'r') as fr:
            content = fr.read()

        req_start = content.find('pub enum KernelRequest')
        req_block = ""
        if req_start != -1:
            req_block = parse_brace_block(content, content.find('{', req_start))

        resp_start = content.find('pub enum KernelResponse')
        resp_block = ""
        if resp_start != -1:
            resp_block = parse_brace_block(content, content.find('{', resp_start))

        f.write("## KernelRequest Variants\n\n")
        f.write("| Variant | Field Types | Source |\n")
        f.write("|---|---|---|\n")

        lines = req_block.split('\n')
        current_variant = None
        for line in lines:
            line = line.strip()
            if not line or line.startswith('//') or line == '{': continue
            if line.startswith('}'):
                current_variant = None
                continue

            if not line.startswith(('pub', 'use', '#')):
                if line.endswith('{'):
                     current_variant = line.rstrip('{').strip()
                elif ':' in line and current_variant:
                     parts = line.split(':')
                     fname = parts[0].strip()
                     ftype = parts[1].strip().rstrip(',')
                     forbidden = check_forbidden(ftype)
                     warn = " **FORBIDDEN: " + ", ".join(forbidden) + "**" if forbidden else ""
                     f.write(f"| `{current_variant}` | `{fname}: {ftype}`{warn} | `abi/src/lib.rs` |\n")
                elif line.endswith(',') and '{' not in line and '(' not in line:
                     f.write(f"| `{line.rstrip(',')}` | Unit | `abi/src/lib.rs` |\n")

        f.write("\n## KernelResponse Variants\n\n")
        f.write("| Variant | Field Types | Source |\n")
        f.write("|---|---|---|\n")

        lines = resp_block.split('\n')
        current_variant = None
        for line in lines:
            line = line.strip()
            if not line or line.startswith('//') or line == '{': continue
            if line.startswith('}'):
                current_variant = None
                continue

            if line.endswith('{'):
                 current_variant = line.rstrip('{').strip()
            elif ':' in line and current_variant:
                 parts = line.split(':')
                 fname = parts[0].strip()
                 ftype = parts[1].strip().rstrip(',')
                 forbidden = check_forbidden(ftype)
                 warn = " **FORBIDDEN: " + ", ".join(forbidden) + "**" if forbidden else ""
                 f.write(f"| `{current_variant}` | `{fname}: {ftype}`{warn} | `abi/src/lib.rs` |\n")
            elif line.endswith(',') and '{' not in line and '(' not in line:
                 f.write(f"| `{line.rstrip(',')}` | Unit | `abi/src/lib.rs` |\n")

        f.write("\n## Wire Types (abi/src/wire/**)\n\n")
        f.write("| Type Name | Kind | Fields/Types | Source | Compliance |\n")
        f.write("|---|---|---|---|---|\n")

        for root, dirs, files in os.walk("abi/src/wire"):
            for file in files:
                if not file.endswith(".rs"): continue
                path = os.path.join(root, file)
                with open(path, 'r') as fr:
                    content = fr.read()
                    items = parse_enums_structs(content, path)
                    for item in items:
                        desc = ""
                        if 'field' in item: desc = f"{item['field']}: {item['type']}"
                        elif 'variant' in item: desc = f"{item['variant']}: {item['type']}"

                        warn = "✅"
                        if item['forbidden']:
                            warn = "❌ " + ", ".join(item['forbidden'])

                        f.write(f"| `{item['name']}` | {item['kind']} | `{desc}` | `{path}` | {warn} |\n")


def generate_schema_authority():
    print("Generating docs/contracts/schema_authority.md...")

    core_schemas = []

    lib_path = 'thing_models/src/lib.rs'
    if os.path.exists(lib_path):
        with open(lib_path, 'r') as f:
            content = f.read()

        iterator = re.finditer(r'schemas\.push\(\((.*?), (.*?), (.*?)\)\);', content)
        for match in iterator:
            kind = match.group(1)
            desc = match.group(2)
            core_schemas.append((kind, desc))

    with open('docs/contracts/schema_authority.md', 'w') as f:
        f.write("# Schema Authority Ledger\n\n")
        f.write("## Core Schemata (Kernel-Known)\n\n")
        f.write("Defined in `thing_models`, registered by `kernel` at boot.\n\n")
        f.write("| Kind (Source) | Description (Source) | Registration |\n")
        f.write("|---|---|---|\n")

        for kind, desc in core_schemas:
            f.write(f"| `{kind}` | `{desc}` | `thing_models/src/lib.rs` |\n")

        f.write("\n## Package Schemata\n\n")
        f.write("- **Registration:** Scoped to the calling process/package via `SYSCALL_SCHEMA_REGISTER_PACKAGE`.\n")
        f.write("- **Storage:** Stored in the kernel's Graph Schema Store.\n")
        f.write("- **Conflicts:** Returns `SchemaRegistryOutcome::Conflict` or `AlreadyRegisteredSame` if a schema with the same Kind but different definition exists.\n")
        f.write("- **Fingerprinting:** SchemaGet returns a 64-bit fingerprint (currently placeholder 0 in `kernel/src/lib.rs`).\n")


def generate_nonos():
    print("Generating docs/contracts/nonos_report.txt...")

    # (pattern, description, scope_path)
    queries = [
        ("mem::transmute", "mem::transmute calls", "."),
        ("&'static str", "&'static str in ABI request/response", "abi/src/lib.rs"),
        ("&'static [", "&'static slices in ABI response", "abi/src/lib.rs"),
        ("static mut", "static mut in user drivers", "user/drivers"),
        ("unsafe impl Send", "unsafe Send in user drivers", "user/drivers"),
        ("unsafe impl Sync", "unsafe Sync in user drivers", "user/drivers"),
        ("Vec<", "Vec usage in wire types", "abi/src/wire"),
        ("trait ", "Traits in ABI", "abi"),
    ]

    with open('docs/contracts/nonos_report.txt', 'w') as f:
        f.write("No-Nos Grep Report\n==================\n\n")

        for pattern, desc, scope in queries:
            f.write(f"--- {desc} ---\n")

            # Check if scope exists
            if not os.path.exists(scope) and scope != ".":
                 f.write(f"Scope {scope} does not exist.\n\n")
                 continue

            try:
                result = subprocess.run(["rg", "--color=never", "--line-number", pattern, scope], capture_output=True, text=True)
                lines = result.stdout.splitlines()

                f.write(f"Count: {len(lines)}\n")
                for line in lines:
                    f.write(line + "\n")

            except FileNotFoundError:
                res = subprocess.run(["grep", "-r", "-n", pattern, scope], capture_output=True, text=True)
                lines = res.stdout.splitlines()
                f.write(f"Count: {len(lines)}\n")
                for line in lines:
                    f.write(line + "\n")

            f.write("\n")

if __name__ == "__main__":
    generate_syscalls()
    generate_abi_surface()
    generate_schema_authority()
    generate_nonos()
