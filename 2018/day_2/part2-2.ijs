#!/usr/local/bin/j
d=.>cutLF CR-.~fread'input.txt'
NB. sort, group into sliding window of pairs
NB. take head and tail of each pair, 
NB. ~: not equal them, then sum to get count of different chars
NB. find the entries with one difference, everything else is 0 
NB. use this to copy out only the pair with '1'
ap=. {. ((1=({."2 +/"1@:~: {:"2)) # [ ) 2 ]\ (d/:d)
NB. print just the chars that match
echo (({: =/"0 {.) # {. ) ap
exit 0
