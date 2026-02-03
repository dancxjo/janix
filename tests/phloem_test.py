import socket
import time
import sys
import subprocess
import os

HOST = '127.0.0.1'
PORT = 2323

def connect():
    for i in range(60):
        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.settimeout(2)
            s.connect((HOST, PORT))
            return s
        except (ConnectionRefusedError, socket.timeout, OSError):
            time.sleep(1)
            if i % 5 == 0:
                print(f"Retrying connection... {i}")
    return None

def main():
    print(f"Connecting to {HOST}:{PORT}...")
    s = connect()
    if not s:
        print("Failed to connect")
        sys.exit(1)

    print("Connected!")

    # Read banner
    try:
        banner = s.recv(1024).decode(errors='ignore')
        print("Banner:", banner)
    except socket.timeout:
        print("Timeout waiting for banner")

    # Helper to send/recv
    def run_cmd(cmd):
        print(f"> {cmd}")
        s.sendall(cmd.encode() + b"\n")
        total_data = ""
        while True:
            try:
                chunk = s.recv(1024).decode(errors='ignore')
                if not chunk: break
                total_data += chunk
                if "gql>" in total_data: # wait for prompt
                    break
            except socket.timeout:
                break
        print(f"< {total_data.strip()}")
        return total_data

    # MERGE node
    res = run_cmd('MERGE (n:TestNode {val: "hello"}) RETURN n')
    if "ok:" not in res or "(id:" not in res:
        print("MERGE failed")
        sys.exit(1)

    # MATCH node
    res = run_cmd('MATCH (n:TestNode) RETURN n')
    if "ok:" not in res or "TestNode" not in res:
        print("MATCH failed")
        sys.exit(1)

    # SET prop
    res = run_cmd('SET n.newprop = 123')
    if "ok:" not in res:
        print("SET failed")
        sys.exit(1)

    # MERGE edge
    run_cmd('MERGE (a:NodeA {name: "A"})')
    run_cmd('MERGE (b:NodeB {name: "B"})')
    res = run_cmd('MERGE (a)-[:LINKS_TO]->(b) RETURN a, b')
    if "ok: merged edge" not in res:
        print("MERGE edge failed")
        sys.exit(1)

    # MATCH edge
    res = run_cmd('MATCH (a)-[:LINKS_TO]->(b) RETURN a, b')
    if "ok:" not in res or "NodeA" not in res or "NodeB" not in res:
        print("MATCH edge failed")
        sys.exit(1)

    run_cmd('QUIT')
    s.close()
    print("Test passed")

if __name__ == "__main__":
    main()
