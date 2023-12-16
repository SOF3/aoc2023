include "d3-lib";

. as $grid |
    find_numbers |
    map(
        select(has_adjacent_gear($grid)) |
        .value | tonumber
    ) |
    add
