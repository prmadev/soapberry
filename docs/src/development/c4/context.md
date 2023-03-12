# Context-Level Structure

## RedMaple 

A Redmaple is just a data-structure with a set of methods.
The implementaition of a RedMaple will contain the business logic. 
Any redmaple can be hosted inside a (not yet created) ForestElf,
Which is a wrapper and manager for  one or more RedMaples.
Each ForestElf implement a set of server-application protocols and is a distinct service application.
When a user wants to interact with the Redmaple Cluster,
they would need to find specific Redmaples.
An ElfGuide keeps a live database of different redmaples and in which ForestElf they are located. 
As such a client application would have to first contact an ElfGuide. 

Each redmaple is commanded by ForestElf synchronously, but the ForestElfs contact each other asynchronously.

```ascii

            +------+ +------+ +------+ +------+          
            | user | | user | | user | | user |          
            +------+ +------+ +------+ +------+          
               | ^     ^ |      ^ \       ^ |            
               | |     | |       \ \      | |            
               v |     | v        \ v     | v            
            +---------------+    +---------------+       
            |   ElfGuide    |    |   ElfGuide    |       
            +---------------+    +---------------+       
                    ^    ^                 ^             
+------------------/------\----------------|------------+
| RedMaple Cluster/        v               |            |
|                /    +--- --------+       |            |
|               v     |  ForestElf |       v            |
|  +--- --------+     |+----------+|     +--- --------+ |
|  | ForestElf  |     || RedMaple ||     |  ForstElf  | |
|  |+----------+|     |+----------+|     |+----------+| |
|  || RedMaple ||<--->|+----------+|<--->|| RedMaple || |
|  |+----------+|     || RedMaple ||     |+----------+| |
|  |+----------+|     |+----------+|     |+----------+| |
|  || RedMaple ||     |+----------+|     || RedMaple || |
|  |+----------+|     || RedMaple ||     |+----------+| |
|  |+----------+|     |+----------+|     |+----------+| |
|  || RedMaple ||     +------------+     || RedMaple || |
|  |+----------+|<---------------------->|+----------+| |
|  +------------+                        +------------+ |
+-------------------------------------------------------+ 

```

Now, this is just how I imagine an architecture based on RedMaples may look-like.
This is not prescriptive.

<figure ><img src="../../assets/2021.jpg"> </img> <figcaption > Randal Munroe, Attribution-NonCommercial 2.5 Generic (CC BY-NC 2.5)</figcaption> </figure>


## WhirlyBird

WhirlyBird is just a  crate full of different implementations of Redmaple.
Each redmaple is Generic over DomainEvents logic and its ViewModes.
Both of which are and should be defind by the domain events and their projectors.
Whirlybird provides a few different ones, but you can just take them to be an example of how you could define your own.

```
+-------------------------------------------------------+
|                       WhirlyBird                      |
|                                                       |
|                                                       |
|                                                       |
|                                                       |
+-------------------------------------------------------+
                   |                          |          
                   |                          |          
                   |                          |          
                   |                          |          
 +-----------------|--------------------------|---------+
 | +---------------v---------+ +--------------v-------+ |
 | | +---------------------+ | | +------------------+ | |
 | | |                     | | | |                  | | |
 | | |                     | | | |                  | | |
 | | |    Domain events    | | | |  Kinds of views  | | |
 | | |                     | | | |                  | | |
 | | |                     | | | |                  | | |
 | | +---------------------+ | | +------------------+ | |
 | +-------------------------+ +----------------------+ |
 |                        RedMaple                      |
 +------------------------------------------------------+

```

## Kyushu

Kyushu just follows a simple server-client architecture.


```                                                                                         
 +---------------------------------------+         +-------------+
 |            Kyushu Server              |         |             |
 | +-----------------------------------+ |-------->|             |
 | |                                   | |         |             |
 | |                                   | |         |             |
 | |                                   | |         |             |
 | |   Journal (One Giant RedMaple)    | |         |Kyushu Client|
 | |                                   | |         |             |
 | |                                   | |         |             |
 | |                                   | |         |             |
 | +-----------------------------------+ |         |             |
 |                                       |<--------|             |
 +---------------------------------------+         +-------------+
                                                      |      ^    
                                                      |      |    
                                                      |      |    
                                                      |      |    
                                                      |      |    
                                                      v      |    
                                               +-----------------+
                                               |                 |
                                               |                 |
                                               |                 |
                                               |      User       |
                                               |                 |
                                               |                 |
                                               |                 |
                                               +-----------------+
```

The key here is that kyushu client is responsible for a implementing the projector of events. 
In the kyushu server, For each journal, we only use one Redmaple.