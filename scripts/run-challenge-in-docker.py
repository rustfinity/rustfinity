#!/usr/bin/env python3
import argparse
import base64
import subprocess
import sys
from pathlib import Path

def read_and_encode(file_path):
    with open(file_path, 'rb') as f:
        return base64.b64encode(f.read()).decode('utf-8')

def main():
    parser = argparse.ArgumentParser(description='Run a Rustfinity challenge in Docker')
    parser.add_argument('challenge', nargs='?', default='printing-hello-world',
                       help='Challenge name (default: printing-hello-world)')
    parser.add_argument('--image', default='rustfinity-runner:staging',
                       help='Docker image to use (default: rustfinity-runner:staging)')
    parser.add_argument('--n-tests', type=int, default=1,
                       help='Number of times benchmarks should run (default: 1)')
    parser.add_argument('--build', action='store_true',
                       help='Build the Docker image before running')

    args = parser.parse_args()

    workspace_root = Path(__file__).parent.parent
    challenge_dir = workspace_root / 'challenges' / args.challenge

    if not challenge_dir.exists():
        print(f"Error: Challenge '{args.challenge}' not found at {challenge_dir}", file=sys.stderr)
        sys.exit(1)

    code_file = challenge_dir / 'src' / 'lib.rs'
    tests_file = challenge_dir / 'tests' / 'tests.rs'
    cargo_toml_file = challenge_dir / 'Cargo.toml'
    
    for file_path, name in [(code_file, 'code'), (tests_file, 'tests'), (cargo_toml_file, 'Cargo.toml')]:
        if not file_path.exists():
            print(f"Error: {name} file not found: {file_path}", file=sys.stderr)
            sys.exit(1)
    
    code_b64 = read_and_encode(code_file)
    tests_b64 = read_and_encode(tests_file)
    cargo_toml_b64 = read_and_encode(cargo_toml_file)
    
    if args.build:
        build_cmd = [
            'docker', 'build',
            '-f', str(workspace_root / 'crates' / 'rustfinity-runner' / 'Dockerfile'),
            '-t', args.image,
            str(workspace_root)
        ]
        build_result = subprocess.run(build_cmd)
        if build_result.returncode != 0:
            print(f"Error: Docker build failed", file=sys.stderr)
            sys.exit(1)
    
    docker_cmd = [
        'docker', 'run', '-i',
        '--rm',
        '--network=none',
        '--cpus=1',
        '-m=500m',
        args.image,
        '/bin/bash', '-c',
        f"/app/rustfinity-runner test --code '{code_b64}' --tests '{tests_b64}' --cargo-toml '{cargo_toml_b64}' --n-tests {args.n_tests}"
    ]
    
    subprocess.run(docker_cmd)

if __name__ == '__main__':
    main()
