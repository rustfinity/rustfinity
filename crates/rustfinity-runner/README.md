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

## Dependency allowlist

The runner container is started with `--network=none`, so `cargo` cannot reach
crates.io while a submission is running. **Every crate a challenge depends on
must already be baked into this image**, at the exact version the image
vendors.

The allowlist is the `cargo add` lines in the [Dockerfile](./Dockerfile):

- `syn`
- `quote`
- `tempfile@3.23.0`
- `tokio@1.53.1`, features `tokio/full,tokio/test-util`

plus the [`syntest`](../syntest/) path crate, which is copied in separately.

A challenge whose `Cargo.toml` pins a different version of one of these, or
names a crate not on the list, compiles fine locally and then fails for every
real user with an offline resolution error.

Adding a crate is a deploy, not a code change:

1. Add it to a `cargo add` line in the Dockerfile, with an exact version.
2. Rebuild and smoke test a challenge that uses it:

   ```sh
   make build TAG=staging
   cd ../.. && ./scripts/run-challenge-in-docker.py <slug> --image rustfinity-runner:staging
   ```

3. Tag and push the image (see [How to deploy](#how-to-deploy) below).
4. Update `RUSTFINITY_RUNNER_IMAGE` in the API environment. The default in
   `apps/api/src/config.rs` is only a fallback and does not follow new tags.

Challenges that need the new crate stay broken in production until step 4
lands. See [docs/tracks.md](../../../../docs/tracks.md) for how this constrains
authoring a whole track.

## How to deploy

Create a new tag and push to github.

### Tag format

- For main `v0.1.0`
- For staging `v0.1.0-staging`
