<h1 align="center">
  SoapBerry
</h1>


<p align="center">
  <img alt="builds" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-build.yml?color=%23eed49f&label=build&logo=Rust&logoColor=%23eed49f&style=for-the-badge">
  <img alt="formaters" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-fmt.yml?color=%23a6da95&label=formatted&logo=Rust&logoColor=%23a6da95&style=for-the-badge">
  <img alt="lints" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-clippy.yml?color=%23f5bde6&label=lints&logo=Rust&logoColor=%23f5bde6&style=for-the-badge">
  <img alt="tests" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-test.yml?color=%23f0c6c6&label=tests&logo=Rust&logoColor=%23f0c6c6&style=for-the-badge">
</p>

<p align="center">
  <img width="200" src="./assets/soapberry.png" alt="a picture of a soapberry tree in the style of a pixel art">
</p>

<p align="center">
  A constellation of event-driven libraries and application(s), including journaling (WIP), microblogging (TBA), and task managing (TBA). 
  But for now, it's non-functional.

</p>


## Documentation
<p align="center">
  <a href="https://amirography.github.io/soapberry/">
    <img alt="tests" src="https://img.shields.io/static/v1?label=documentation&message=mdbook&color=c6a0f6&logo=Rust&logoColor=c6a0f6&style=for-the-badge">
  </a>
  <a href="https://docs.rs/redmaple/latest/redmaple/">
    <img alt="RedMaple's API Documentation" src="https://img.shields.io/static/v1?label=RedMaple&message=docs.rs&color=f5a97f&logo=Rust&logoColor=f5a97f&style=for-the-badge">
  </a>
  <a href="https://docs.rs/whirlybird/latest/whirlybird/">
    <img alt="WhirlyBird's API Documentation" src="https://img.shields.io/static/v1?label=WhirlyBird&message=docs.rs&color=f5a97f&logo=Rust&logoColor=f5a97f&style=for-the-badge">
  </a>
</p>

> *Yes. I know! This page looks like a unicorn has been exploded in the candy factory here. Bare with me to complete the documentation in time. These colors will probably look less like the face of a kid left alone in the art shop.* 

- [**The Book**](https://amirography.github.io/soapberry/): Currently there is a very pretty online book that is
  there to document the development and over-arching
  decisions related to all the projects under soapberry.
  Though it is bare right now, it should get pretty thick in time.
  - it currently contains the following informations:
    - Architectural Decision Records (ADRs).
    - How an `mdBook` can *actually* become pretty.

- **API Documentation**: API documentations per crates are provided for libraries:
  - [RedMaple's API Documentation](https://docs.rs/redmaple/latest/redmaple/) 
  - [WhirlyBird's API Documentation](https://docs.rs/whirlybird/latest/whirlybird/) 
 
  

## Project Structure

### Applications

#### [`crates/kyushu`](./crates/kyushu) :notebook_with_decorative_cover:
<img alt="kyushu's license" src="https://img.shields.io/crates/l/kyushu?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/kyushu?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)  

A journaling client-server application
which is based on contemplating on events being inside different journeys.

### Libraries

####  [`crates/whirlybird`](./crates/whirlybird) :fallen_leaf:

<img alt="whirlybird's license" src="https://img.shields.io/crates/l/whirlybird?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/whirlybird?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)

A set of implementations for `RedMaple`.
this is where you can implement bussiness logics on top of `RedMaple`. 

#### [`crates/redmaple`](./crates/redmaple) :deciduous_tree: 
<img alt="redmaple's license" src="https://img.shields.io/crates/l/redmaple?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/redmaple?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)

A very basic data-structure which is formed for
holding **event-sourced** and **event-first** data. 

## Current status

I try to update the status of the project rigorously. However, there are many badges here that are automatically updated.


### Development


<img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/y/amirography/soapberry?color=%238bd5ca&logo=git&logoColor=%238bd5ca&style=for-the-badge">

The project is under active development.
Although I often commit changes, I'm working on the overall design of the project now.
And more than code's I'm researching and experimenting with different forms.
I consider this project to be a long-term one.
So, I'm not in hurray to release it. 

For now, consider the whole project to be in **pre-alpha stage**.




### Dependencies

<p align="left">
  <img alt="up to date" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-outdated.yml?color=%237dc4e4&label=up-to-date&logo=Rust&logoColor=%237dc4e4&style=for-the-badge">
  <img alt="no unused dependencies" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-udep.yml?color=%23a6da95&label=no-unused&logo=Rust&logoColor=%23a6da95&style=for-the-badge">
  <img alt="license consistency" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-deny.yml?color=%238aadf4&label=licenses-compatible&logo=Rust&logoColor=%238aadf4&style=for-the-badge">
</p>

I use libraries. I Cannot help it. But I can:
1. Keep them updated.
2. Trim-down the unused ones.
3. Make sure that I my license is compatible with theirs.


### Security

<p align="left">
  <img alt="advisory audit" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-audit.yml?color=%23eed49f&label=audits&logo=Rust&logoColor=%23eed49f&style=for-the-badge">
  <img alt="secure dependencies" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-pants.yml?color=%23f0c6c6&label=secure-dependencies&logo=Rust&logoColor=%23f0c6c6&style=for-the-badge">
</p>

I take security pretty seriously. You should too.
Although I do not always have the resources to make sure every CVE is resolved with my dependencies, I will:
1. make sure that I only include those that have good track record.
2. not hide any potential vulnerability in my dependencies.
3. update my dependencies as soon as a security update is out. 

## Contributions

Currently the only contributor of this project is myself,
but as soon as I stabilize the fundamental structure of the project, 
I will accept Pull Requests from others, and make a `CONTRIBUTING.md` file.
For now please use [code of conduct](/CODE_OF_CONDUCT.md) for all other regards.

