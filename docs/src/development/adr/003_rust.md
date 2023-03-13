# Use Rust

|INFO| VALUE|
|:---:|:---|
| DATE | 2023-03-10|
| AUTHOR | Amir H. Alesheikh <amirhossein.alesheikh@gmail.com>|
| PROPOSER | Amir H. Alesheikh <amirhossein.alesheikh@gmail.com>|
| DECIDER | Amir H. Alesheikh <amirhossein.alesheikh@gmail.com>| 
| CONSULTED |NONE|
| STATUS | accepted|
 <!--proposed | accepted | rejected | superseded by <example.adoc> | deprecated-->

## Decision:

use Rust as the main language

## Context

- What should be the main language for implementation of soapberry?

## Decision drivers

- Speed of development
- Type-safety
- Expressiveness
- Tooling
- Runtime-Error proneness

## Considered Alternatives

### Go

- Considering Go's mark-and-sweep garbage-collection strategy and  the memory intensiveness of Event-Sourcing, Go potentially can cause a considerable headache in scale. 
- Go's type-system is very small and lacks many features which is neccassary for a good compile-time gurantee of correctness. To be specific, lack of sum-types means type-casting at runtime.
- Go also lacks a mature generic support which makes the anything more than the most basic generic functions to be exponentially less-readable.


### Other languages

- All the languages with mark-and-sweep garbage collectors are suffering from the GC problems mentioned.
- All the languages with manual memory-management (c, c++, and even zig) are making a trade-off that I would not and need not wanted to make.

## Consequences


### Pros

- More correctness gurantees.
- No concerns over future memory problems.
- Very expressive and condence code.


### Cons

- Finding the right abstraction takes more time. 
- Compilation-time and LSP speed lacks when compared to Go.

