include "d3-lib";

. as $grid |
    (index("\n") + 1) as $width |
    find_numbers as $numbers |
    find_gears |
    map(
        (
            [
                . - $width - 1, . - $width, . - $width + 1,
                . - 1, . + 1,
                . + $width - 1, . + $width, . + $width + 1
            ] |
            map(select(. >= 0))
        ) as $adjacent |
        (
            $numbers |
            map(
                select(
                    . as $number |
                    $adjacent | any(($number.start <= .) and (. < $number.end))
                ) |
                .value | tonumber
            )
        ) |
        select(length == 2) |
        .[0] * .[1]
    ) |
    add
