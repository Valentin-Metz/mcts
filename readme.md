# Connect Four

An interactive [Connect Four](https://en.wikipedia.org/wiki/Connect_Four) command-line game,
where two players take turns dropping colored discs into a grid.

The first player to connect four discs of the same color wins.

### Implementation:

The NPC-Ai utilizes [Monte Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search)
to heuristically search for moves.

Monte Carlo search trees provide the foundation for more sophisticated machine learning algorithms,
such as [AlphaZero](https://en.wikipedia.org/wiki/AlphaZero).

Rollout is currently performed by random simulation.
For a simple game such as connect four, this already yields an agent of significant strength.

### Usage:

```bash
cargo run --release
```
