# part 1

    |'B'1.:'F0.:'R1.:'L0.:2öZ

    | -> split lines to array
     'B'1.: -> replace all B with 1
           'F0.: -> replace all F with 0
                'R'1.: -> replace all R with 1
                      'R0.: -> replace all L with 0
                          2ö -> convert to base 2
                            Z -> max

# part 2

    |'B'1.:'F0.:'R1.:'L0.:2öD<KW>

    | -> split lines to array
     'B'1.: -> replace all B with 1
           'F0.: -> replace all F with 0
                'R'1.: -> replace all R with 1
                      'R0.: -> replace all L with 0
                          2ö -> convert to base 2 
                            D-> dup
                             < -> decrement all values in second copy
                              K -> Push a without b's (i.e the only seats without a value one higher)
                               W -> smallest one (we dont want the bad value at the end)
                                > -> inc by one to get the missing seat  

# both + the AND 4 (not zero) trick...

    |εÇ4&0QJ}2öZ,D<KW>,
