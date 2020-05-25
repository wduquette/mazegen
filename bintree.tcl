# TCL implementation of the binary tree maze algorithm.

proc bintree {name rows cols} {
    grid $name $rows $cols
    set ncells [$name cells]

    for {set cell 0} {$cell < $ncells} {incr cell} {
        set neighbors [list]
        set cnorth [$name cellto $cell north]
        set ceast [$name cellto $cell east]

        if {$cnorth ne ""} {
            lappend neighbors $cnorth
        }

        if {$ceast ne ""} {
           lappend neighbors $ceast
        }

        if {[llength $neighbors] != 0} {
            $name link $cell [rand sample $neighbors]
        }
    }

    return $name
}

bintree m1 10 20

puts [m1 text]
