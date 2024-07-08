# faster_lcs

### Observations:
- A LcsField is a value in the table who's index ends up being used in the LCS itself.
- Between every LcsField is always {Length of source input} many normal values.
- An LcsField is always the top left occuring instance of that value in the field. (?)

### Usefull sites: 
- https://www.mimuw.edu.pl/~erykk/algovis/lcs.html
- https://planetcalc.com/9499/


### Info

If i have this lcs table:

```
 | AIEAAOE
-+--------
I| 0111111
A| 1112222
E| 1122223
O| 1122233
A| 1123333
E| 1123334
A| 1123444
E| 1123445
```
And i re-organise it like this:

```
I| AIEAAOE A| AIEAAOE E| AIEAAOE O| AIEAAOE A| AIEAAOE E| AIEAAOE A| AIEAAOE ...
I| 0111111 A| 1112222 E| 1122223 O| 1122233 A| 1123333 E| 1123334 A| 1123444 ...

E| AIEAAOE E| AIEAAOE
E| 1123445 E| 1123445
```

And then i remove every value from each row except the first one that's different from the previous row

```
 | AIEAAOE
-+--------
I| 0100000
A| 1000000
E| 0020000
O| 0000030
A| 0003000
E| 0000004
A| 0000400
E| 0000005
E| 0000000
-+--------
 | 1123435
```
Then i eventually get a special row of numbers

This special row of numbers when filtered so only values larger then the last left over value remain looks like this:
```
 | 1123435
-+--------
 | 1 234 5
```

when i then take the index of each leftover index and access the target input with it. i get, the LCS!
```
 | AIEAAOE
-+--------
 | 1123435
-+--------
 | 1 234 5
-+--------
 | A EAA E
```