include "d2-parse";

def solve_line:
    parse_line.sets |
    map({
        (.[0].color): map(.count) | max,
    }) |
    add |
    .red * .green * .blue
;

. / "\n" | map(solve_line) | add
