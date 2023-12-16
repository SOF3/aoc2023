include "util";

def find_numbers:
    . as $grid |
    [range(length)] |
    map(
        select(
            (get_char($grid) | is_digit)
            and (. == 0 or (. - 1 | get_char($grid) | is_digit | not))
        ) | {
            start: .,
            end: until(get_char($grid) | is_digit | not; . + 1),
        } | {
            start: .start, end: .end,
            value: $grid[.start:.end],
        }
    )
;

def has_gear:
    . as $string |
    [range(length)] |
    any(
        get_char($string) |
        . != "."
        and . != "\n"
        and (is_digit | not)
    )
;

def has_adjacent_gear($grid):
    . as $found |
    ($grid | index("\n") + 1) as $width |
    [-$width, 0, $width] |
    any(
        $grid[($found.start + . - 1 | greater(0)) : ($found.end + . + 1 | greater(0))] |
        has_gear
    )
;

def find_gears:
    . as $string |
    [range(length)] |
    map(select(get_char($string) | has_gear))
;
