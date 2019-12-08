#!/usr/local/bin/j
layers=.100 150 $ 1!:1 < 'input.txt'
count_n=.[: +/"1 =
min=.[ {~ [: (i. <./) ]
p1=.(count_n&'1' * count_n&'2') @ (min count_n&'0')
echo p1 layers
echo ''
blend=.[: {. [ {~ [: I. '2'&~:
p2=.[: {&' @' '1' = 6 25 $ blend"1 @ |:
echo p2 layers
exit ''
