

## Todo

### CLI
- Install Clap
  - Jump to game type from the cli
  - helpful man pages?

### Storage engine
- Make `serde` serializability part of the `Play` interface
- Figure out how to do game history/event sourcing efficently
- Have ltti write that ish down to a sqlite db

### Testability
- Figure out a set of macros to automatically write tests for `Play` interface invariants that can't be tested in the type system
  - Game Determinism (recreating game state from events)
  - Spectator/Player View can be determined at any time or created from the base case and updates

### Tic Tac Toe Gui
- Make tic tac toe gui prettier
  - embolden winning squares
  - put `X` and `O`s in the squares for the color blind

### Games

#### Rock Paper Scissors
- Try to make rps to prove out the `Play` interface for games with simultaneous turns

#### Crazy Eights
- Try to make ce to prove out the `Play` interface for games with secret info and variable number of players

#### Marooned
- This one is basically big tic tac toe with slightly more involved settings, I already have a decent implemenation for this one

### General
- Figure out how players work, it seems like the player/user translation should happen at much higher level than `Play` or `GameRunner`
- Port over the `common` library from the old ltti
  - Cards
  - Dice
  - Chess pieces
