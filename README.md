# reconnect
Reconciliation tool built on Rust

I believe recon must be cheap and fast and I inspired by the anti-entropy usage of Merkle trees by Cassandra, I had a fully functional reconciliation engine written in Python a year ago.  

This project has the following additives :

1. Code rewritten in Rust.  No GC and absolutely bare metal. 
2. Redesigned the Merkle Tree to be an array (like the heap array) instead of a first class datastucture (with Python), to optimize on locality.
3. Enhanced the code to support CSVs. Has abstractions to extend to other formats.
4. The currently implementation stores the hashes (32 bit) in memory.  However, the underlying data could be stored in an external store (database/file). Also, swapped Adler with murmur3 for speed.
5. Finally, the numbers.  1 million * 1 million compares in ~22 seconds. The code is single threaded and runs on a single machine.  Changing it to multi-threaded is a very quick change.  Can be sharded too but needs some work.
