# Biggun'

An arcade style fishing game.

## Gameplay

Bring the hook to the fish's mouth and wrangle it with WASD. Pull the fish to
the surface using SPACE. When a target score is reached, you will move forward
to the next stage.

## Project Organization

`biggun_game` is a simple crate that takes the plugins created in `biggun_lib`
and configures the app to run the game. It also contains all of the game's
assets. The hope is that doing this increases code cohesion and decreases
unnecessary tight coupling.

Cross-system communication should mainly be done through *Bevy events*. Bevy
has a good way of figuring out how to easily call with all of the arguments
that a system's function needs.

Many Bevy projects mirror the bevy repository, having a crate for each major
system. This does achieves the same thing and is likely better for larger
projects, but the systems in this project are relatively small and too dependent,
so I don't believe it is worth all of the extra organization effort.
The project obeys more of the *Cargo workspace* project structure described in
The Rust Book.
