# grid -- Maze Grid Object

A grid, represented in Rust as the `Grid` struct, is a rectangular array of *cells*, each of which
can be linked to the cells north, south, east, and west of it.  The process of creating a maze
involves "carving out" links within a grid.

In TCL, a grid is represented as a Molt object: a molt command whose subcommands correspond more
or less to the methods of the `Grid` struct.  The [**grid** constructor](#constructor) creates
instances of the grid [object command](#object-command).

## Cell Coordinates

Cells in the grid are identified by their matrix-style coordinates, (*i,j*), where *i* ranges from
0 to (num_rows - 1) and *j* ranges from 0 to (num_cols - 1). Cell (0,0) is at the top-left;
rows *i* extend down, and the columns *j* extend to the right.

## Cell Directions

A cell can have up to four neighbors, which are to the **north**, **south**, **east**, or
**west**.

## Constructor

**Syntax: grid *name* *rows* *cols***

The **grid** command creates a grid object, a Molt binding to a Rust `Grid` struct.  The
new grid is a Molt command called *name*; the command provides access to the newly created
grid, which will have the given number of rows and columns.  Returns the *name*.

```tcl
$ grid mygrid 10 20
mygrid
$ mygrid rows
10
$ mygrid cols
20
$
```

## Object Command

**Syntax: *grid subcommand* ?*args*...?**

The *grid* object command has the following subcommands.

| Subcommand                          | Description |
| ----------------------------------- | ----------- |
| [*grid* cells](#grid-cells)         | The number of cells in the grid  |
| [*grid* cellto](#grid-cellto)       | The coordinates of the cell in a given direction |
| [*grid* clear](#grid-clear)         | Clears the grid, i.e., unlinks all linked cells  |
| [*grid* cols](#grid-cols)           | The number of columns in the grid |
| [*grid* deadends](#grid-deadends)   | Dead-end cells |
| [*grid* distances](#grid-distances) | Distances of all cells from a given cell |
| [*grid* linked](#grid-linked)       | Are two cells linked? |
| [*grid* linkedto](#grid-linkedto)   | Is a cell linked to the cell in a given direction? |
| [*grid* link](#grid-link)           | Links two adjacent cells |
| [*grid* links](#grid-links)         | The cells to which a cell is linked |
| [*grid* longest](#grid-longest)     | The longest path in the grid |
| [*grid* neighbors](#grid-neighbors) | The cells adjacent to a given cell |
| [*grid* path](#grid-path)           | Find a path between two cells |
| [*grid* render](#grid-render)       | Render an image that depicts the grid |
| [*grid* rows](#grid-rows)           | The number of rows in the grid |
| [*grid* text](#grid-text)           | Render a string that depicts the grid |
| [*grid* unlink](#grid-unlink)       | Unlink two adjacent cells |

### *grid* cells
---
**Syntax: *grid* cells**

Returns the number of cells in the grid.

### *grid* cellto
---
**Syntax: *grid* cellto *i j* north|south|east|west**

Returns the ID of the cell to the **north**, **south**, **east** or **west** of cell *i,j*
or the empty string if there is no such cell.

### *grid* clear
---
**Syntax: *grid* clear**

Clears all links from the grid, returning it to its initial state.

### *grid* cols
---
**Syntax: *grid* cols**

Returns the number of columns in the grid.

### *grid* deadends
---
**Syntax: *grid* deadends ?-flat|-pairs?**

Returns a list of the coordinates of all dead-end cells in the grid.  A cell is a dead-end
if it is linked to only one other cell.  By default, or if **-flat** is given, the list is a
flat list "*i1 j1 i2 j2...*".  If **-pairs** is given, the list is a list of pairs,
"{*i1 j1*} {*i2 j2*}...".

```tcl
# Cells (0,0) and (15,9) are dead ends.
puts [$grid deadends]   ;# outputs "0 0 15 9"

# Prints all dead ends, one per line
foreach {i j} [$grid deadends] {
    puts "Dead End: $i,$j"
}
```

### *grid* distances
---
**Syntax: *grid* distances *i j* -flat|-pairs**

Computes the minimum distance from cell *i,j* to every other cell.  By default, or if the
**-flat** option is given, returns a flat list of cell coordinates and distances,
"*i1 j1 dist1 i2 j2 dist2*...".  If the **-pairs** option is given, returns a list
"{*i1 j1*} *dist1* {*i2 j2*} *dist2*...".  Note that in the latter case, the result is
a Molt dictionary of distances by coordinate pair.

```tcl
# Distances from cell 0,0 as a list
set list [$grid distances 0 0]

foreach {i j distance} $list {
    puts "Distance to $i,$j = $distance"
}

# Distances from cell 0,0 as a dictionary

array set distances [grid distances 0 0 -pairs]
set distanceTo35 $distances({3 5})
```

### *grid* link
---
**Syntax: *grid* link *i1 j1 i2 j2***

Links the cell *i1,j1* to *i2,j2* given their IDs.  The two cells must be neighbors.  Note:
links are always bidirectional.

```tcl
$ $grid link 5 6 5 7
$ $grid linked 5 6 5 7
1
$ $grid linked 5 7 5 6
1
```

### *grid* linked
---
**Syntax: *grid* linked *i1 j1 i2 j2***

Returns true if cell *i1,j1* is linked to *i2,j2*, and false otherwise.

### *grid* linkedto
---
**Syntax: *grid* linkedto *i j* north|south|east|west**

Returns true if the cell *i,j* is linked to its neighbor in the given direction.

### *grid* links
---
**Syntax: *grid* links *i j* ?-flat|-pairs?**

By default, or if the **-flat** option is given, returns a flat list of the coordinates of the
cells that are linked to *i j*.  If **-pairs** is given, returns a list of pairs.

For example, suppose that 0,0 is linked east to 0,1 and south to 1,0:

```
$ $grid links 0 0
0 1 1 0
$ $grid links 0 0 -flat
0 1 1 0
$ $grid links 0 0 -pairs
{0 1} {1 0}
```

### *grid* longest
---
**Syntax: *grid* longest ?-flat|-pairs?**

Returns the longest path through the grid.  By default, or if **-flat** is given, returns a flat
list of the coordinates of the cells; if **-pairs**, returns a list of pairs.

Supposing the longest path starts at 0,0,

```
$ $grid longest
0 0 0 1 1 1 ...
$ $grid longest -pairs
{0 0} {0 1} {1 1} ...
```

### *grid* neighbors
---
**Syntax: *grid* neighbors *i j* ?-flat|-pairs?**

Returns a list of the coordinates of the cells that are neighbors of cell *i,j*.  By default, or
if **-flat** is given, returns a flat list of the coordinates; if **-pairs** is given, returns a list
of pairs.

```
$ $grid neighbors 5 5
4 5 6 5 5 4 5 6
$ $grid neighbors 5 5 -pairs
{4 5} {6 5} {5 4} {5 6}
```

### *grid* path
---
**Syntax: *grid* path *i1 j1 i2 j2* ?-flat|-pairs?**

Returns a path from cell *i1,j1* to cell *i2,j2* as a list of cell coordinates starting with
*i1,j1* and proceeding in order to *i2,j2*, or the empty list if there is no such path. By
default, or if **-flat** is given, the list is a flat list of the coordinates of the cells;
if **-pairs**, the list is a list of pairs.

### *grid* render
---
**Syntax: *grid* render *filename* ?*options...*?**

Renders the grid as an image, saving it to the file with the given name.  Valid options are as follows:

| Option                    | Description |
| ------------------------- | ----------- |
| **-cellsize *pixels***    | A cell's height and width in pixels.  Defaults to 10. |
| **-borderwidth *pixels*** | The width of the border between cells, in pixels.  Defaults to 1. |

### *grid* rows
---
**Syntax: *grid* rows**

Returns the number of rows in the grid.

### *grid* text
---
**Syntax: *grid* text ?*options...*?**

Returns a string that represents the maze in ASCII characters.  The options are as follows:

| Option                  | Description |
| ----------------------- | ----------- |
| **-cellwidth *chars***  | A cell's width in monospace characters.  Defaults to 3. |
| **-autowidth *margin*** | Size cells to the data, leaving a *margin*.  Defaults to 1. |
| **-flat *list***        | A flat list of cell coordinates and data strings to include in each cell |
| **-pairs *dict***       | A dictionary of data strings by cell coordinate pair |

The caller can provide data to be written into the cells. The data is given in one of two forms:

* As a **-flat** list "*i1 j1 data1 i2 j2 data2 ...*". To leave a cell empty, omit its data.

* As a list of **-pairs** and data strings, "{*i1 j1*} *data1* {*i2 j2*} *data2*...".  Note that
  this is also a dictionary of data values by coordinate pair.  To leave a cell empty, omit its
  data.

The **-cellwidth** gives the actual width of each cell in monospace characters; if data is
given, it will be truncated to fit the width.  If **-autowidth** is also given, the width will
be set to the length of the longest value plus twice the *margin*, so that the data can be
presented without truncation.  In this case, the **-cellwidth** becomes the minimum width.

```tcl
$ grid mygrid 3 5
...
$ mygrid text
+---+---+---+---+---+
|                   |
+   +---+---+   +   +
|   |       |   |   |
+   +---+   +---+   +
|           |       |
+---+---+---+---+---+

$ mygrid text -autowidth 1 -flat [mygrid distances 0 0]
+---+---+---+---+---+
| 0   1   2   3   4 |
+   +---+---+   +   +
| 1 | 6   5 | 4 | 5 |
+   +---+   +---+   +
| 2   3   4 | 7   6 |
+---+---+---+---+---+
$
```

### *grid* unlink
---
**Syntax: *grid* unlink *i1 j2 i2 j2***

Unlinks the two cells (if they were linked).  The two cells must be neighbors.  Note:
all links are bidirectional; unlinking *i1,j1* from *i2,j2* also unlinks *i2,j2* from *i1,j1*.
