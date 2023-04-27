# Type Semantics

I extensively use Type-Driven Development (TyDD).

This means that of the time the logic of the programs happens trough type transformations.

However there are broader type changes where we are not just checking for business logic but we are trying to adhere to implementation limitations.

An example of that is converting types from enums into more protobuffer friendly types.

This means that there are architectural patterns in type conversions that I will outline using the bellow diagram:

<figure ><img src="../../assets/type_semantics.svg"> </img> <figcaption > </figcaption> </figure>
