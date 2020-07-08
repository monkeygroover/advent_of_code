#!/usr/local/bin/jconsole
d=. ".>cutLF fread'input.txt' NB. read file, cut at each linefeed, unbox (>) then convert strings to numbers (".)
a=.0>._2+<.@%&3  NB. 
b=.0:`(]+$:@a)@.*
echo +/(a,.b)a d
exit ''
