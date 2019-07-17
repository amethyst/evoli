# Evoli
A micro-ecosystem simulation game, progressively designed and developed as an official showcase project for the Amethyst engine. The current iteration of the game (v0.1.1 and onwards) simulates a few different species occupying the same, limited space.

For more information about the current game design and our goals and history so far, read [our introduction](https://community.amethyst.rs/t/evoli-introduction/770).

## Media

![may-10](https://raw.githubusercontent.com/amethyst/evoli/master/evoli-shot.png) 

## Install / Play

If you are compiling on Linux, you need to install some dependencies first. They are necessary to compile and run the Amethyst engine. Please follow the instructions in the [Amethyst README](https://github.com/amethyst/amethyst#dependencies).

Ensure you have Cargo installed ([use rustup if you don't](https://rustup.rs/)), and run the following:

```
cargo run
```

If you run into issues please report them here or on http://discord.gg/amethyst in the #showcase-game channel.

## Profiling
We use the same profiling library Amethyst uses. Run the game with
```
cargo run --release --features profiler
```
then exit the game without a crash to generate a file `thread_profile.json`.
See the Amethyst instructions [Profiling the engine](https://github.com/amethyst/amethyst/blob/master/docs/CONTRIBUTING.md#profiling-the-engine) on
how to use that file.

Search the code for `profile_scope` for an example on how to add profiling markers to the code.

## Get involved

- [Sitemap doc](https://community.amethyst.rs/t/evoli-sitemap/771) - All essential reading and communication tools.
- [Development conventions](https://community.amethyst.rs/t/evoli-development-conventions/783)
- [Contribution doc](https://community.amethyst.rs/t/evoli-is-ready-for-contributions/815)

## License

Split license: Choose between [Apache or MIT license](https://github.com/amethyst/evoli/blob/master/LICENSE.md).
