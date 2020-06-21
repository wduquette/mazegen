# TCL implementation of the binary tree maze algorithm.

proc bintree {name rows cols} {
    grid $name $rows $cols

    for {set i 0} {$i < $rows} {incr i} {
        for {set j 0} {$j < $cols} {incr j} {
            set candidates [list]
            set cnorth [$name cellto $i $j north]
            set ceast [$name cellto $i $j east]

            if {$cnorth ne ""} {
                lappend candidates $cnorth
            }

            if {$ceast ne ""} {
                lappend candidates $ceast
            }

            if {[llength $candidates] != 0} {
                $name link $i $j {*}[rand sample $candidates]
            }
        }
    }

    return $name
}

bintree m1 10 20

puts [m1 text]
