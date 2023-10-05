# Connect Four

An interactive [Connect Four](https://en.wikipedia.org/wiki/Connect_Four) command-line game,
where two players attempt to be the first to connect four markings of their color.

### Implementation:

This implementation provides an Ai opponent,
which utilizes [Monte Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search)
to search and learn optimal moves in self-play at **runtime**.

Monte Carlo search trees provide the foundation for more sophisticated reinforcement learning algorithms,
such as [AlphaZero](https://en.wikipedia.org/wiki/AlphaZero),
where node-ratings are performed by a neural network.

Rollout is currently performed by random simulation.
For a simple game such as connect four, this already yields an agent of significant strength.

### Usage:

```bash
cargo run --release
```
