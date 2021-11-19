# LibTableTop (ltt)

### What is LibTableTop?

LibTableTop is a set of rust libraries and binaries providing fast/correct/hackable implementations of board games.

### Why build LibTableTop?

LibTableTop's goal is to provide correct/extensible/hackable implementations of board games, along with the tooling to analyze and push our understanding of board game strategy.

## I want to...

### Write a bot to play a game

TODO

### Write my own game

TODO

### Contribute to LibTableTop

TODO

## Project Structure

### LibTableTop for Players/Bot Builders/Hobbyists

| Component       | Uses                                                                        |
| :--             | :--                                                                         |
| `ltti`          | CLI toolkit for working with lttgames                                       |
| `lttcore`       | Base traits and helper utilities for working with the rest of lib table top |
| `games/ltt*`    | Games maintained by the ltt team                                            |

### Internal LibTableTop components for more complex/unique/interesting things

| Internal Component | Uses                               |
| :--                | :--                                |
| `lttnetworking`    | Networking protocol/state machines |
| `lttruntime`       | Async runtime                      |
