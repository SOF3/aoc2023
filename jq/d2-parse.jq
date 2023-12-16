def parse_line:
    capture("Game (?<game>\\d+): (?<sets>.*)") |
    (
        .sets / "; " |
        map(
            . / ", " | map(
                capture("(?<count>\\d+) (?<color>[a-z]+)") |
                {color, count: .count | tonumber}
            )
        ) |
        add |
        group_by(.color)
    ) as $sets |
    (.game | tonumber) as $game |
    {game: $game, sets: $sets}
;
