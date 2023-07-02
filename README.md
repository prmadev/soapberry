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
Event-sourced libraries and binary for aggregating, shaping, and journaling, as inspired by the wisdom of maple trees.
</p>

## What of it

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

Welcome to the enchanting realm of Soapberry, a captivating GitHub repository that brings forth a trio of remarkable crates, each bearing the name of a majestic maple tree analogy.
Within this repository, you will discover the boundless potential of event-driven systems, elegantly manifested through the Rust programming language.
Together, these masterful creations of Soapberry invite you to embark on a journey of innovation and craftsmanship, crafting resilient and immersive event-driven architectures that resonate with the beauty and wisdom of maple trees


### Libraries

#### [`crates/redmaple`](./crates/redmaple) :deciduous_tree: 
<img alt="redmaple's license" src="https://img.shields.io/crates/l/redmaple?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/redmaple?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)

At the core of Soapberry lies RedMaple,
a versatile and robust crate boasting a generic datatype specifically crafted to hold event aggregates with grace and efficiency.

####  [`crates/whirlybird`](./crates/whirlybird) :fallen_leaf:

<img alt="whirlybird's license" src="https://img.shields.io/crates/l/whirlybird?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/whirlybird?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)

As RedMaple sets the stage, Whirlybird gracefully emerges, presenting a crate adorned with a rich assortment of domain logic tools.
Through its wisdom, Whirlybird shapes and molds the events held within RedMaple, empowering developers to navigate the intricate nuances of event processing and transformation.


### Applications

#### [`crates/kyushu`](./crates/kyushu) :notebook_with_decorative_cover:
<img alt="kyushu's license" src="https://img.shields.io/crates/l/kyushu?color=f4dbd6&label=license&style=flat-square"> ![Crates.io](https://img.shields.io/crates/v/kyushu?color=8bd5ca&label=version&logo=rust&logoColor=8bd5ca&style=flat-square)  

Completing this exquisite trio is Kyushu, a local-first, event-sourced CLI application that embraces the essence of journalling.
With Kyushu, you can effortlessly capture and track events, embracing a seamless and intuitive CLI experience.


## Current status

In the realm of Soapberry,
I meticulously update project status, while automated badges dynamically reflect its evolving nature.


### Development


<img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/y/amirography/soapberry?color=%238bd5ca&logo=git&logoColor=%238bd5ca&style=for-the-badge">

Engaged in active development, I currently prioritize the project's overarching design.
Beyond mere code, I delve into diverse forms, conducting research and experimentation.
Embracing a long-term perspective, I refrain from rushing its release.
Presently, consider the project to reside in the pre-alpha stage.



### Dependencies

<p align="left">
  <img alt="up to date" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-outdated.yml?color=%237dc4e4&label=up-to-date&logo=Rust&logoColor=%237dc4e4&style=for-the-badge">
  <img alt="no unused dependencies" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-udep.yml?color=%23a6da95&label=no-unused&logo=Rust&logoColor=%23a6da95&style=for-the-badge">
  <img alt="license consistency" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-deny.yml?color=%238aadf4&label=licenses-compatible&logo=Rust&logoColor=%238aadf4&style=for-the-badge">
</p>

Inevitably reliant on libraries, I diligently undertake several measures:
1. I diligently update them to ensure their optimal functionality.
2. I meticulously trim down any unused libraries, streamlining the project.
3. I conscientiously ensure compatibility between my chosen license and the libraries employed.

### Security

<p align="left">
  <img alt="advisory audit" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-audit.yml?color=%23eed49f&label=audits&logo=Rust&logoColor=%23eed49f&style=for-the-badge">
  <img alt="secure dependencies" src="https://img.shields.io/github/actions/workflow/status/amirography/soapberry/cargo-pants.yml?color=%23f0c6c6&label=secure-dependencies&logo=Rust&logoColor=%23f0c6c6&style=for-the-badge">
</p>

Imbued with a deep sense of responsibility towards security, I earnestly emphasize its significance.
While I may not possess boundless resources to address every CVE within my dependencies, I commit to the following principles:
1. Meticulously selecting dependencies with a proven track record, ensuring their reliability and robustness.
2. Transparently acknowledging any potential vulnerabilities present in my dependencies, refusing to conceal them.
3. Swiftly updating my dependencies upon the release of security updates, prioritizing the safeguarding of the project's integrity.

## Contributions

Being the sole contributor to this project at present,
my primary objective revolves around stabilizing its fundamental structure. Once accomplished,
I eagerly anticipate welcoming Pull Requests from fellow collaborators, accompanied by the inclusion of a `CONTRIBUTING.md` file.
In the meantime, I kindly request adhering to the established [code of conduct](/CODE_OF_CONDUCT.md) for all other matters related to the project.
