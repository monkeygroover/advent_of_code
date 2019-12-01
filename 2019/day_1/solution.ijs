#!/usr/local/bin/j
d=. ".&>cutLF CR-.~fread'input.txt'
fuel =: 3 : 0
0>._2+<.%&3 y
)
rec_fuel =: 3 : 0
    0:`(] + $:@fuel)@.* fuel y
)
echo +/fuel d
echo +/rec_fuel d
exit ''
