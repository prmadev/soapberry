# Context-Level Structure

## RedMaple 

A RedMaple, if you will, can be conceived as a wrapper encompassing a veritable array of events.
These events, residing within the library, demonstrate their versatility through the employment of generics, thereby allowing for a multitude of implementations to coexist harmoniously.

Now, let us delve into the crux of RedMaple's design, wherein lies the implementation of each event, serving as the vessel for essential business logic.
To fulfill this crucial role, I have opted for the utilization of Whirlybird, a tool specifically tailored to facilitate the seamless execution of said logic.

It is worth noting that the RedMaple's domain extends further, enabling its integration within the overarching framework of a ForestElf.
Functioning as both a wrapper and a diligent manager, the ForestElf takes charge of one or more RedMaples, ensuring their smooth operation within the system.

For individuals seeking to engage with the Redmaple Cluster, a meticulous search for specific RedMaples becomes imperative.
By identifying and locating the desired RedMaples, users gain the means to interact fruitfully with this interconnected network.

Let us bear in mind, however, that the depiction I have presented here reflects a hypothetical architecture based on RedMaples, devoid of any prescriptive directives.

<figure ><img src="../../assets/2021.jpg"> </img> <figcaption > Randal Munroe, Attribution-NonCommercial 2.5 Generic (CC BY-NC 2.5)</figcaption> </figure>


## WhirlyBird

WhirlyBird can be aptly likened to a treasure trove—a crate teeming with a diverse array of event implementations and their corresponding projectors tailored explicitly for Redmaple.
The essence of Redmaple lies in its inherent generality, as it gracefully embraces the events and their projectors in a generic fashion.

These events and projectors hold paramount importance, as they are not only pivotal components of Redmaple but should also be thoughtfully crafted within the framework of the domain itself.
WhirlyBird generously offers a selection of distinct event implementations and projectors, serving as valuable exemplars for inspiration in devising one's own bespoke solutions.

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

Kyushu adheres to a straightforward and pragmatic local-first architecture.

```                                                                                         
 +----------------------------------------------------+
 |            Kyushu                                  |
 | +---------------------------+                      |
 | |                           |                      |
 | |                           |                      |
 | |                           |                      |
 | | A lot of  small RedMaples |                      |
 | |                           |                      |
 | |                           |                      |
 | |                           |                      |
 | +---------------------------+                      |
 |                                                    |
 +----------------------------------------------------+
      |       ^                            |      ^    
      |       |                            |      |    
      |       |                            |      |    
      |       |                            |      |    
      |       |                            |      |    
      v       |                            v      |    
  +--------------+                   +-----------------+
  |              |                   |                 |
  |              |                   |                 |
  |              |                   |                 |
  |    FileDB    |                   |      User       |
  |              |                   |                 |
  |              |                   |                 |
  |              |                   |                 |
  +--------------+                   +-----------------+
```

