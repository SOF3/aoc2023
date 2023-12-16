. / "\n" | map(
    select(. != "") |
    . / "" |
    map(tonumber?) |
    .[0] * 10 + .[-1]
) |
add
