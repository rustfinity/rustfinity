In this challenge, you will implement a **maze solver in Rust**. A maze can be represented as a **grid where each cell is either an open path or a wall**. The goal is to navigate from a starting point to an ending point using the shortest path possible. This problem will test your understanding of control flow in Rust, including loops, conditionals, and possibly recursion.

## Your Task

Your task is to write a function `solve_maze` that takes a **2D vector of characters representing the maze and two tuples representing the start and end points.** The maze will be given as a grid of characters where `'S'` represents the start, `'E'` represents the end, `'.'` represents an open path, and `'#'` represents a wall.

**The function should return a vector of tuples** representing the path from start to end if a path exists, or an empty vector if no path exists.

## Constraints

- The maze will always contain exactly one `'S'` and one `'E'`.
- You can only move **up, down, left, or right**.
- Implement the function using appropriate control flow constructs **(loops, conditionals, and/or recursion)**.
- Ensure your solution **handles edge cases**, such as no available path or mazes with various sizes.
- If the maze is a dead-end, return an empty vector.
- If the maze has multiple solutions, return the shortest path.

## Example

```rust
let maze = vec![
    vec!['S', '.', '#', '#', '#'],
    vec!['#', '.', '#', '.', '.'],
    vec!['#', '.', '.', '.', '#'],
    vec!['#', '#', '#', '.', '#'],
    vec!['#', '#', '#', 'E', '#']
];
let start = (0, 0);
let end = (4, 3);

let path = solve_maze(maze, start, end);
assert_eq!(
    path,
    vec![
        (0, 0), // starting point
        (0, 1), // right
        (1, 1), // down
        (2, 1), // down
        (2, 2), // right
        (2, 3), // right
        (3, 3), // down
        (4, 3) // down
    ]
);
```

In the example above, we start from `'S'` at `(0, 0)`, the first move would be going to the right `(0, 1)`, then down `(1, 1)`, and so on until we reach the end at `(4, 3)`.

> Did You Know?
>
> Maze solving algorithms are not just for games. They are used in robotics for **pathfinding**, in computer networks for routing, and in various optimization problems. Algorithms such as Depth-First Search (DFS), Breadth-First Search (BFS), and A\* are commonly used to solve these problems efficiently.

## Hints

1. **Collections**:
   - `VecDeque`: A double-ended queue from the `std::collections` module, which is useful for implementing a queue for BFS. Methods like `push_back` and `pop_front` will be helpful.

2. **Indexing**:
   - Use `usize` for indices and be cautious with arithmetic operations to avoid overflow. The `wrapping_add` method can help with safe arithmetic.

3. **2D Vector Initialization**:
   - Initialize 2D vectors for `visited` and `path` tracking. Use nested `vec!` macros for creating the initial structure.

4. **Backtracking Path**:
   - Once the end point is reached, backtrack using the `path` vector to reconstruct the path from end to start. Collect these coordinates in a vector and reverse it to get the path from start to end.

5. **Boundary Checks**:
   - Ensure the new coordinates are within the maze boundaries and check if the cell is a wall (`'#'`) or already visited.
