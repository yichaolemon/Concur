# Concur

A fully-featured, personalized concurrency library.

The library should make the following operations and examples easy to implement:

- Stream data through several operations and output results
- Stream data through filters or 'reduce' phases
- Download a file by requesting several chunks of it in parallel and write it serially to disk
- Traverse a tree structure and performing an operation in parallel on all nodes of the tree
- Promises -- request an object asynchronously, pass it into a pipeline, and wait for the result
- Propagate an error from any point in the pipeline