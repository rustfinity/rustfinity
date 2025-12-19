# Rustfinity code runner

The official code runner used in [Rustfinity.com](https://www.rustfinity.com), a platform to learn Rust programming language.

## How to use

It's best to run this in a docker container to be completely isolated from the outside world.

1. Build the image locally

   ```sh
   make build TAG=latest
   ```

2. Convert your code to **base64** format.

   In JavaScript it would be something like this

   ```js
   const code = `fn main() {
       println!("Hello, world!");
   }`

   const base64Code = Buffer.from(code).toString("base64")
   ```

3. Run the code using the CLI

```bash
docker run -i \
    --rm \
    --network=none \
    --cpus=1 \
    -m=500m \
    rustfinity-runner \
    /bin/bash -c "/app/rustfinity-runner playground --code 'cHViIGZuIGhlbGxvX3dvcmxkKCkgewogICAgcHJpbnRsbiEoIkdvb2Qgam9iLCB5b3UgZGVjb2RlZCBpdCA6RCIpCn0K'"
```

### Commands

- `test`: Runs a [rustfinity challenge](../../challenges/), requires a few arguments:
  - `--code`: Base64 encoded code (user submitted)
  - `--tests`: Base64 encoded tests file
  - `--cargo-toml`: Base64 encoded Cargo.toml file for that challenge
  - `--n-tests` (optional): How many times the benchmarks should run (default = 1)

- `playground`: Runs a provided snippet of code, used in [rustfinity.com/playground](https://www.rustfinity.com/playground), requires one argument:
  - `--code`: Base64 encoded code (user submitted)

## How to deploy

Create a new tag and push to github.

### Tag format

- For main `v0.1.0`
- For staging `v0.1.0-staging`
