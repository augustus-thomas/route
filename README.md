## Purpose
This is a small experiment comparing Rust to Python.

## Specification
Goal: pathfind in a weighted graph using the Bellman-Ford algorithm.

Input: 
- nodes.csv: nodes and their x, y, z coordinates in Euclidean space.
- edges.csv: directed edges.
- source & target: stdin

Output: 
- shortest path between nodes; stdout.

Example Input:
Find me a path from cat to meow
```
/path/to/some/executable cat meow
```

Example Output:
The shortest path from cat to meow is ~29 units via b and a.
```
-> cat -> b -> a -> meow 29.83186638883156
```

## Reflection & Comparison
### Similarities
Both Rust and Python rely on third party libraries for reading Command Seperated Value (csv) files. Both use the standard library for writing the standard output.
In both cases the logic of the program is the same: parse the input into a useful data structure representation, process the data structure, output the result. In both cases a dictionary/hashmap was used to represent the directed acyclic graph from source to target. Both implementations relied on data structures: in Python the data structures were built-in, whereas in Rust I had to specify the structures myself.
### Differences
By far the most stark difference is error handling: Rust won't even compile without every edge case being handled at compile time, which makes its code air-tight. I did find that development time was 3 to 4 times longer because of that, although this might be confounded by my understanding/comfort with the language. I also used more lines of code in Rust since it is less expressive and more cumbersome with error catching requirements.

Despite that I think Rust still "wins." Python is a prototyping language which is expressive enough to quickly sketch the behavior of a program. But Rust's error messages and rust-analyzer are so helpful that I think it would be better to write production-quality code in Rust. This is because errors can be caught at compile-time as opposed to hitting exceptions during deployment. 

I liked that Rust managed dependencies in a ``transparent" way. I don't know how many times I have worked on a Python bug only to find that I was using a different dependency version than someone else. The Crates.toml file is very useful in that regard.

I found that the Rust linter could catch unused or unneeded data structures, which would be very good for bug catching in a larger project. The compiler also prefers pass-by-reference over pass-by-value, but enforces data ownership requirements; I do not know enough about this yet, but it encourages me to write cleaner code I think.
