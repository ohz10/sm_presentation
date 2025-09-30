#!/bin/ksh
# guided walk-through of the code; breaking it into
# digestable chunks.
#
# Use numeric keypad to navigate:
# - {up,left}: previous slide
# - {down,right}: next slide

set -A slides \
"bat -r 68:86 src/bin/tennis_naive.rs" \
"bat -r 1:5   src/bin/tennis_naive.rs" \
"bat -r 7:10  src/bin/tennis_naive.rs" \
"bat -r 12:15 -r 66:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 54:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 33:37 -r 52:52 -r 66:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 33:33 -r 39:41 -r 52:52 -r 66:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 33:33 -r 43:52 -r 66:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 17:19 -r 66:66 src/bin/tennis_naive.rs" \
"bat -r 12:12 -r 21:31 src/bin/tennis_naive.rs" \
"clear" \
\
"bat -r 104:122 src/bin/tennis_naive_enum.rs" \
"bat -r 26:29 -r 7:13 -r 1:5   src/bin/tennis_naive_enum.rs" \
"bat -r 15:24   src/bin/tennis_naive_enum.rs" \
"bat -r 31:37 -r 102:102 src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 90:102  src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 65:68 -r 88:88 -r 102:102 src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 65:65 -r 70:78 -r 87:88 src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 65:65 -r 70:70 -r 79:88 src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 39:41 src/bin/tennis_naive_enum.rs" \
"bat -r 31:31 -r 49:63 src/bin/tennis_naive_enum.rs" \




function echo.off {
    stty -echo -icanon
}

function echo.on {
    stty echo icanon
}

function prev {
    if [[ $i -gt 0 ]]; then
        i=$i-1
    fi
}

function next {
    i=$i+1
}

function show_slide {
    eval "${slides[$i]}"
}

function loop {
    integer i=0

    while true; do
        show_slide i

        echo.off
        key=`dd bs=1 count=1 2>/dev/null`
        echo.on

        case "$key" in
            '8') prev i ;;
            '4') prev i ;;
            '2') next i ;;
            '6') next i ;;
            'q') break  ;;
              *) ;;
        esac
    done
}

function run {
    loop
}

run
