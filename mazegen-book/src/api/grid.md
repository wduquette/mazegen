# grid -- Maze Grid Object

A grid, represented in Rust as the `Grid` struct, is a rectangular array of *cells*, each of which
can be linked to the cells north, south, east, and west of it.  The process of creating a maze
involves "carving out" links within a grid.

In TCL, a grid is represented as a Molt object: a molt command whose subcommands correspond more
or less to the methods of the `Grid` struct.  The [**grid** constructor](#constructor) creates
instances of the grid [object command](#object-command).

## Cell IDs and Coordinates

Each cell in the grid has a unique cell ID, an integer from 0 to N-1, where N is the number of
cells in the grid.  A cell can also be referenced by its (*i,j*) row/column coordinates.  Cell
(0,0) is at the top-left; Rows *i* extend down, and the columns *j* extend to the right.

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
| [*grid* cell](#grid-cell)           | Converts an *i j* pair to a cell ID  |
| [*grid* cells](#grid-cells)         | The number of cells in the grid  |
| [*grid* cellto](#grid-cellto)       | The ID of the cell in a given direction |
| [*grid* clear](#grid-clear)         | Clears the grid, i.e., unlinks all linked cells  |
| [*grid* cols](#grid-cols)           | The number of columns in the grid |
| [*grid* deadends](#grid-deadends)   | Cell IDs of dead-end cells |
| [*grid* distances](#grid-distances) | Distances of all cells from a given cell |
| [*grid* i](#grid-i)                 | Converts a cell ID to an *i* coordinate |
| [*grid* ij](#grid-ij)               | Converts a cell ID to an *i j* pair  |
| [*grid* j](#grid-j)                 | Converts a cell ID to a *j* coordinate  |
| [*grid* linked](#grid-linked)       | Are two cells linked? |
| [*grid* linkedto](#grid-linkedto)   | Is a cell linked to the cell in a given direction? |
| [*grid* link](#grid-link)           | Links two adjacent cells |
| [*grid* links](#grid-links)         | The cells to which a cell is linked |
| [*grid* longest](#grid-longest)     | The longest path in the grid |
| [*grid* neighbors](#grid-neighbors) | The cells adjacent to a given cell |
| [*grid* render](#grid-render)       | Render an image that depicts the grid |
| [*grid* rows](#grid-rows)           | The number of rows in the grid |
| [*grid* text](#grid-text)           | Render a string that depicts the grid |
| [*grid* unlink](#grid-unlink)       | Unlink two adjacent cells |

### *grid* cell
---
**Syntax: *grid* cell *i j***

Returns the cell ID corresponding to row *i*, column *j*.  Rows are indexed from 0 to M-1,
and columns are indexed from 0 to N-1.

### *grid* cells
---
**Syntax: *grid* cells**

Returns the number of cells in the grid.

### *grid* cellto
---
**Syntax: *grid* cellto *cell* north|south|east|west**

Returns the ID of the cell to the **north**, **south**, **east** or **west** of the given *cell*,
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
**Syntax: *grid* deadends**

Returns a list of the IDs of all dead-end cells in the grid.  A cell is a dead-end if it
is linked to only one other cell.

### *grid* distances
---
**Syntax: *grid* distances *cell* -list|-dict**

Computes the minimum distance from the given *cell* to every other cell.  By default, or if the
`-list` option is given, returns a list of distances by cell ID.  If the `-dict` option is given,
returns a Molt dictionary of distances by cell ID.

```tcl
$ set list [$grid distances 0]                   ;# Distances from cell 0 as a list
$ set to35 [lindex $list 35]                     ;# Distance from 0 to cell 35
$ array set distances [grid distances 0 -dict]   ;# Distances from cell 0 as a dict
$ set to35 $distances(35)                        ;# Distance from 0 to cell 35
```

### *grid* i
---
**Syntax: *grid* i *cell***

Gets the row index of the *cell* with the given ID.

### *grid* ij
---
**Syntax: *grid* ij *cell***

Gets the row/column coordinates of the *cell* with the given ID as a two-element list.

```tcl
$ set pair [$grid ij 35]      ;# Get the I/J coordinates of cell 35
$ set i [lindex $pair 0]
$ set j [lindex $pair 1]
$ lassign [$grid ij 35] i j   ;# Once Molt implements lassign
```

### *grid* j
---
**Syntax: *grid* j *cell***

Gets the column index of the *cell* with the given ID.

### *grid* link
---
**Syntax: *grid* link *cell1 cell2***

Links the cell *cell1* to *cell2* given their IDs.  The two cells must be adjacent.  Note:
links are always bidirectional.

```tcl
$ $grid link $cell1 $cell2
$ $grid linked $cell1 $cell2
1
$ $grid linked $cell2 $cell1
1
```

### *grid* linked
---
**Syntax: *grid* linked *cell1 cell2***

Returns true if cell *cell1* is linked to *cell2*, given their IDs, and false otherwise.

### *grid* linkedto
---
**Syntax: *grid* linkedto *cell* north|south|east|west**

Returns true if the given *cell* is linked to its neighbor in the given direction.

### *grid* links
---
**Syntax: *grid* links *cell***

Returns a list of the IDs of the cells that are linked to the given *cell*.

### *grid* longest
---
**Syntax: *grid* longest**

Returns the longest path through the grid as a list of cell IDs.

### *grid* neighbors
---
**Syntax: *grid* neighbors *cell***

Returns a list of the IDs of the cells that are neighbors of the given *cell*.

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
**Syntax: *grid* text**

Returns a string that represents the maze in ASCII characters.  TODO: needs the ability
to add labels to cells.

```tcl
$ grid mygrid 3 5
...
$ mygrid text
+---+---+---+---+---+
|       |   |       |
+   +   +   +---+   +
|   |       |       |
+   +---+---+   +---+
|                   |
+---+---+---+---+---+
```

### *grid* unlink
---
**Syntax: *grid* unlink *cell1 cell2***

Unlinks the two cells (if they were linked).  The two cells must be adjacent.  Note:
all links are bidirectional; unlinking *cell1* from *cell2* also unlinks *cell2* from *cell1*.
