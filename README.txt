# TFHE_concurrency
Working concurrency for TFHE in Concrete

Currently parallel bootstraping in concrete using rayon is broken
This implementation makes concurrent bootstrapping easy.
Some additional misc functions are available(or will be)

TODO;
Fix ordering for this solution(currently dosn't guarantee that the ordering of Ciphertext stays the same during the parallel bootstrapping)
Implment concurrent Ciphertext Multiplication
Automatically chosing optimal nbr of threads depending on work load.

