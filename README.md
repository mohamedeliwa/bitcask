# Log-Structured key/value storage engine

It's basically a partial implementation of the Bitcask storage engine as a key-value store, created for learning and practice.

It offers high-performance reads and writes, subject to the requirement that all the keys fit in the available RAM, since the hash map is kept completely in memory.

The values can use more space than there is available memory, since they can be loaded from disk with just one disk seek.

A storage engine like Bitcask is well suited to situations where the value for each key is updated frequently.

In this kind of workload, there are a lot of writes, but there are not too many distinct keys—you have a large number of writes per key, but it’s feasible to keep all keys in memory.

## Storage Format

The underlying storage format is very simple: a text file where each line contains a key-value pair (Record), separated by a comma (roughly like a CSV file, ignoring escaping issues).
