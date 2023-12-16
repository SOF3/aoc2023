def is_digit: [tonumber?] | length > 0;

def get_char($string): $string[. : (. + 1)];

# Returns greater of . and $x
def greater($x):
    if $x > . then $x else . end
;

# Returns less of . and $x
def less($x):
    if $x < . then $x else . end
;
