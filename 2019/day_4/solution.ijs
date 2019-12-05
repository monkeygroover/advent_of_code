#!/usr/local/bin/j
to_text=. ".^:_1
rle=: ;@(<@(":@#);.1~ 2 ~:/\ (a.{.@-.{.),])
d=.138307+i.516198
a=.(to_text&.>) d
b=.(/:~@to_text&.>) d
mon=.(a = b) # a
rl=.(rle&.>) mon
echo rl
exit ''
