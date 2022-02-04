# TFHE_concurrency
Working concurrency for TFHE in Concrete

Currently parallel bootstraping in concrete using rayon is broken
\n This simple solution work. Some additional misc functions are available

TODO;
Fix ordering for this solution(currently dosn't guarantee that the ordering of Ciphertext stays the same during the parallel bootstrapping)
Implment concurrent Ciphertext Multiplication
