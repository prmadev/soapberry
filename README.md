<h1 align="center">
SoapBerry
</h1>

<p align="center">
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/ci.yml?color=%23eed49f&label=test&logo=Rust&logoColor=%23eed49f&style=for-the-badge"> <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/y/amirography/soapberry?color=%23f4dbd6&logo=git&logoColor=%23f4dbd6&style=for-the-badge">
</p>

<p align="center">
<img width="200" src="./assets/soapberry.png" alt="a picture of a soapberry tree in the style of a pixel art">
</p>

## Documentation

- [**The Book**](https://amirography.github.io/soapberry/): Currently there is a very pretty online book that is
  there to document the development and over-arching
  decisions related to all the projects under soapberry.
  Though it is bare right now, it should get pretty thick in time.
  - it currently contains the following informations:
    - Architectural Decision Records (ADRs).
    - How an `mdBook` can *actually* become pretty.
 
  

## Project Structure

### Binaries

- [`crates/kyushu`](./crates/kyushu): A journal client-server application
  which is based on contemplating on events being inside different journeys.  

### Libraries

- [`crates/whirlybird`](./crates/whirlybird): A set of implementations for `RedMaple`.
  this is where you can implement bussiness logics on top of `RedMaple`. 

- [`crates/redmaple`](./crates/redmaple): A very basic data-structure which is formed for
  holding **event-sourced** and **event-first** data.
