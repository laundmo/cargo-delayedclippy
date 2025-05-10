# Cargo Delayed Clippy
Because i cared about those `0.15s` of compile time.

> Why would you want to delay clippy?
When you you want to run clippy "on save", but later.
> Why would you want to do that?
Because you might have something else you want to run first, like [bacon](https://github.com/Canop/bacon)

## When NOT to use this

> I'm seeing `waiting for file lock on build directory` and have a bit of disk space available
Change the Rust-Analyzer target directory, so it stores files separately.

To do this, set `rust-analyzer.cargo.targetDir` to `true` or a specific path in your IDEs settings.
Alternatively, set `CARGO_TARGET_DIR` to a path using `rust-analyzer.cargo.extraEnv`

> I can freely control how i run clippy
I recommend something like `sleep 0.2; cargo clippy` instead.

## When to use this

This was specifically created to avoid the small delays incurred by `waiting for file lock on package cache` when using `bacon run` on projects which compile very quickly. It appears, `bacon` is just a tiny bit slower than Rust-Analyzer, so it always ends up waiting.

Specifically, for my setup, i managed to reduce my compile times by, at most, roughly `0.15s`. This is noticable going from `0.61s` to `0.45s`, but if your compile times are any larger than that, its probably not worth it.

## Usage

After installing (`cargo install cargo-delayedclippy`) set `rust-analyzer.check.command` to `delayedclippy`.

The default delay is `180ms`, which you can configure by using the `DELAYEDCLIPPY_MS` environment variable. (probably in `rust-analyzer.check.extraEnv`)

## Contributing
I hope theres nothing to add.

## Thanks
My ADHD for making me focus on those tenths of a second.