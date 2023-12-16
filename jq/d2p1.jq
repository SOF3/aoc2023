include "d2-parse";

def solve_line:
    parse_line |
    (
        .sets |
        map({
            (.[0].color): map(.count) | max,
        }) |
        add |
        .red <= 12 and .green <= 13 and .blue <= 14
    ) as $possible |
    select($possible) |
    .game | tonumber
;

. / "\n" | map(solve_line) | add
