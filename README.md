# reconnect - comparison/reconciliation tool built on Rust

Inspired by the usage of Merkle trees by Cassandra for detecting anti-entropy, this is an attempt to use Merkle trees for performing quick and efficient comparison of two datasets.  

This project has the following additives :

1. Redesigned the Merkle Tree to be an array (like the heap array) instead of a first class datastucture, to optimize on locality.
2. Enhanced the code to support CSVs. Has abstractions to extend to other formats.
3. The current implementation stores the hashes (32 bit) in memory.  However, the underlying data could be stored in an external store (database/file). Also, swapped Adler with murmur3 for speed.
4. 1 million * 1 million compares in ~22 seconds. The code is single threaded and runs on a single machine.  


**Complexity:**

As against hash/tree based comparisons where the time complexity needs to be calculated based on input, this is an example of [output sensitive algorithm](https://en.wikipedia.org/wiki/Output-sensitive_algorithm) where the complexity is Θ(d), d being the number of leaf node differences.

**Rust experts**

Would greatly appreciate your feedback on idiomatic usage, best practices and optimizations.
