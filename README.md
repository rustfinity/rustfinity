[![Rustfinity Banner](./images/rustfinity-header.png)](https://rustfinity.com)

# Rustfinity

<p>
  <a href="https://rustfinity.com"><img src="https://img.shields.io/badge/Rustfinity%20Docs-18181B?style=flat&logo=rust" alt="Website"></a>
  <a href="https://github.com/dcodesdev/rustfinity.com/issues"><img src="https://img.shields.io/github/issues/dcodesdev/rustfinity.com.svg?style=flat&colorA=18181B&colorB=28CF8D" alt="Issues"></a>
  <a href="https://github.com/dcodesdev/rustfinity.com/pulls"><img src="https://img.shields.io/github/issues-pr/dcodesdev/rustfinity.com.svg?style=flat&colorA=18181B&colorB=28CF8D" alt="Pull Requests"></a>
</p>

Rustfinity is an **interactive learning platform** for Rust developers of all levels.
**[Visit rustfinity.com](https://rustfinity.com)** to explore challenges, rustlings, and level up your Rust skills!

## Features

- **Challenges**: Hands-on coding exercises from basics to advanced topics
- **Rustlings**: The classic rustlings exercises, now available in the browser
- **CLI**: Download and solve challenges locally

## Folder Structure

```
.
├── challenges/      # Coding challenges served on rustfinity.com
│   └── challenge-x/
│       ├── description.md
│       ├── src/lib.rs, starter.rs
│       └── tests/tests.rs
└── crates/
    ├── cli              # Download & submit challenges locally
    ├── rustfinity-runner # Runs tests in secure Docker containers
    └── syntest          # Syntax-based testing using Rust AST
```

## Contribute

1. **[Open an issue](https://github.com/dcodesdev/rustfinity.com/issues)** for bugs or suggestions
2. **Fork & PR** to submit changes
3. **[Join Discord](https://discord.gg/8GRcUqY48B)** to discuss ideas

## Local Development

```bash
git clone https://github.com/dcodesdev/rustfinity.com.git
cd rustfinity.com
cargo build
```

## Follow Us

<p>
  <a href="https://x.com/rustfinity"><img src="https://img.shields.io/badge/X-18181B?style=flat&logo=x&logoColor=white" alt="X"></a>
  <a href="https://github.com/dcodesdev/rustfinity.com"><img src="https://img.shields.io/badge/GitHub-18181B?style=flat&logo=github&logoColor=white" alt="GitHub"></a>
  <a href="https://discord.gg/8GRcUqY48B"><img src="https://img.shields.io/badge/Discord-18181B?style=flat&logo=discord&logoColor=white" alt="Discord"></a>
</p>

## License

[Rustfinity Proprietary License](https://github.com/dcodesdev/rustfinity.com/blob/main/LICENSE)
