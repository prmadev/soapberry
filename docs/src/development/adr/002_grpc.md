# Use  gRPC for synchronous messaging

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

use gRPC for synchronous messaging

## Context

What kind of APIs should we for synchronous messaging?

## Decision drivers

- Speed of development
- Type-safety
- File transfer
- Ease of documentation

## Considered Alternatives

### RESTful

- Documentation needs external tools (such as OpenAPI)
- Does not handle streaming data well
- Even though it is ubiquitous everyone uses it differently from one another.
   Which makes it harder to learn, seeing that everyone has their own preconceived notion of how it supposed to work.

### GraphQL

- Does not offer anything over RESTful that matters to the project, but increases the complexity considerably

### Messaging queues

- messaging queues only make sense here in the context of asynchronous messaging. Which is not applicable here.

## Consequences


### Pros

- Improve ease of development and documentation
- Decrease concerns over type-safety
- Boon to the performance
- much easier to stream files and periodic data

### Cons

- limits the number of languages supported as a client
- increases complexity of working in-case a client is written in JavaScript, which needs a complex proxies, like envoy.
- different workflow from RESful and GraphQL

