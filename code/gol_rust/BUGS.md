## Bugs in Rust GoL   
These are in (rough) increasing order of difficulty  
**Line 97**: Compiler error must be fixed to build  
**Line 33**: Render board uses the same character for alive and dead cells  
**Line 103**: The input is not trimmed, so the q operation will not quit the program  
**Line 46**: This calculates the neighbor indices, and will miss all neighbors “below” the current cell  
**Line 81**: Row and column indices are swapped, which will lead to an incorrect calculation of neighbors  