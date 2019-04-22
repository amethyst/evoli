# Evolution Island

### Motivation

In our pursuit to make [amethyst.rs](https://amethyst.rs) a flagship game engine for Rust, the Amethyst Foundation will be backing a small handful of "Showcase Games" that effectively demonstrate the Amethyst engine's key capabilities. What this "backing" entails differs depending on the project. Evolution Island is "backed" by the foundation in that it is being designed and developed in-house by the Amethyst team itself.

### Brief description

Imagine a 2D [terrarium](https://en.wikipedia.org/wiki/Terrarium). A tiny ecosystem enclosed in a sealed container. Inside this closed ecosystem exists three types of living creatures: Plants, Herbivores and Predators. The goal of the player is to maintain the balance (for 2 minutes or, say, 2 weeks, depending on the game mode) of the ecosystem, not letting any entity type go completely extinct. In later iterations of the game the player will be able to use mutations to generate a practically infinite amount of entity variations, and with that a new goal metric becomes available: Create the greatest entity diversity possible without the ecosystem collapsing.

Later still there will be multiplayer applications to play with, but we'll be focusing on single-player for some months still.

Read the complete description in our [MVP Design Doc](https://community.amethyst-engine.org/t/demo-game-evolution-island-mvp/487)

### Screen captures

![](https://community.amethyst.rs/uploads/default/original/1X/51b1b68b786dd211703b74f1be4fbe044f105b26.png) 

https://vimeo.com/331507073

## Contributing

The current state of the game implementation is explained in our [Dev Planning topic](https://community.amethyst.rs/t/demo-game-evolution-island-initial-prototype-dev-planning/537). In short, we've made good progress and now we'd like to open up the project to anyone interested in contributing. See our [0.1 MVP milestone](https://github.com/amethyst/evolution-island/milestone/1) for available issues.

Don't worry too much about whether you know enough about Rust or game coding yet to contribute. A good way to get started is to run the game, peruse the source code and start asking us questions about stuff you want to understand better.

**Note:** Amethyst 0.11 is coming soon, so most new implementation work should probably wait until then. We're still in "high friction mode" so be prepared to make stuff, learn from it then throw it out for something better. Iteration is the name of the game here.


## Build Instructions
Type on your favorite terminal emulator: 
```
cargo run
```

If you run into issues please report them here or on http://discord.gg/amethyst.
