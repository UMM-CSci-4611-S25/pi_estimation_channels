# Estimating pi in parallel using channels

The basic structure here is:

- A set of threads that generate random 2D points in the square [-1,1] x [-1,1].
  These points are then sent via channels to…
- A (small) set of threads (maybe 2?) that organize the points into one of four quadrants:
  bottom left ([-1, 0] x [-1, 0]), bottom right ([0, 1] x [-1, 0]),
  top left ([-1, 0] x [0, 1]), or top right ([0, 1], [0, 1]).
  These points are then sent via channels to…
- A set of four threads (one per quadrant) that each count points
  that are inside the unit circle centered at (0, 0), and the points
  that are outside that circle. Every 1,000 points, they reset their
  counters and send a summary (number inside and number outside) to…
- The final thread, which updates the display of the current estimate
  of pi.

This is almost certainly profound overkill for this problem, but it
illustrates how we might use channels to communicate between parallel
processes.

Probably the biggest "mistake" in this design is the "middle" nodes
that classify points into quadrants. It's not clear that splitting
points into quadrants actually buys us anything beyond giving us an
"obvious" way to organize the final four nodes. We could just have
an arbitrary number of nodes in that "layer" and just distribute the
connections evenly from the first "layer" of nodes to that "layer".

In more complex problems, however, you often want to collect
"related" data into a single channel/node as that can simplify
some of the processing of that data. So this structure could
definitely make sense in a more complex space, and it provides
a useful example of using channels.

I'm going to assume (without the sort of performance testing you'd
actually want here) that

- The generation of random numbers in the first "layer" is expensive.
  Rust uses cryptographically secure random number generation by
  default, which is a fairly expensive process. We could use other
  crates that provide cheaper random number generation, but that's
  for another day.
- The classification in the middle "layer" is very cheap. This is
  just a few comparisons, and should be quite a lot cheaper than
  generating cryptographically secure random numbers.
- The "counting" in the final layer is somewhere between these two
  in complexity, but probably closer to the random number
  generation. We'll have to compute the distance between point
  and the origin, which will involve several floating point
  multiplications. If we also computed the square root (which isn't
  necessary here since we're using the unit circle), these could
  become even more expensive.

So I'm going to estimate the number of cores (using `std::thread::available_parallelism()`). If we call
that `N`, I'll create:

- 2 threads for the middle layer that organizes points into
  quadrants
- N/4 threads for the first layer that generates the points
- N/4 threads for the final layer that counts the points

This has us using roughly half the cores on the host computer,
which is pretty reasonable, leaving enough cores that the
OS can still be responsive. If we want to crank up
the performance, we could go to N/3 or even N/2 threads for
the first and third layers and see what that does.

---

## Results

The following results are for running 10 million samples in `--release` mode,
using `hyperfine` to estimate the times:

| **Method** | **Time** |
|------------|----------|
| Serial | 200ms |
| One generator, one classifier | 450ms |
