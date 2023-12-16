def words: [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
def digits: [range(1; 10)] | map(tostring);
def all_digits: ([words, [range(1; 10)]] | transpose) + ([digits, [range(1; 10)]] | transpose);

def find(indexer; comparator):
    . as $line |
    all_digits | map(
        .[1] as $value |
        {line: $line, digit: .[0]} | indexer |
        select(. != null) |
        {pos: ., value: $value}
    ) |
    min_by(.pos | comparator) |
    .value
;

def solve_line:
    select(. != "") |
    find(
        .digit as $key |
        .line | index($key);
        .
    ) * 10
    + find(
        .digit as $key |
        .line | rindex($key);
        -.
    )
;

. / "\n" | map(solve_line) | add
