# Use Workspaces

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

Using one repo, which hosts a workspace,
and many crates containing binary and library crates.

## Context

How should we structure different crates related to the soapberry project?

## Decision drivers

- Ease of change
- Reliability of API
- Complexity of maintenance

## Considered Alternatives

### One repo for soapberry and submodules for each crate

- Using submodules will create complexity when updating versions
- Also this will not help with immediate feedback

### one repo per crate

- This item will cause a huge amount of friction when one crate updates.
- Also this will cause a headache for maintenance of CI/CD, considering that each change needs to be replicated on all crates.



## Consequences

### Pros

- Increases the agility.

### Cons

- History of repose that I have already created will be lost.
