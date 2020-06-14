# maze -- Maze Constructor

The `maze` command creates [**grid**](grid.md) objects containing mazes produced one of a
number of algorithms.  This command is a stopgap; in the long run, there will be a distinct
command for each useful kind of maze, with appropriate options.

## maze
---
**Syntax: maze *algorithm* *name* *rows* *columns***

Creates a grid object called *name* containing a maze produced by the given *algorithm*.  The
new grid will have the given number of *rows* and *columns*.  The **backtracker** algorithm
is the best of these for practical use, but could be elaborated for even better results.  (I've
got some Java code somewhere that parameterizes it nicely.)

The algorithms are as follows:

| Algorithm       | Description |
| --------------- | ----------- |
| **backtracker** | A good basic maze with good river. |
| **bintree**     | A dirt simple, not very satisfactory maze. |
| **huntandkill** | Similar to **backtracker**; better memory usage, but slower. |
| **sidewinder**  | Slightly better than **bintree** |
