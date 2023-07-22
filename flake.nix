{
  description = "Soapberry";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      rustWithMuslTarget = pkgs.rust-bin.stable.latest.default.override {
        targets = ["x86_64-unknown-linux-musl"];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustWithMuslTarget;

      kyushu_items = craneLib.crateNameFromCargoToml {cargoToml = ./crates/kyushu/Cargo.toml;};
      kyushu = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        name = kyushu_items.pname;
        version = kyushu_items.version;
        pname = kyushu_items.pname;
        CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
        doCheck = true;
      };
    in {
      checks = {
        inherit kyushu;
      };

      packages.kyushu = kyushu;
      packages.default = kyushu;

      apps.default = flake-utils.lib.mkApp {
        drv = kyushu;
      };
    });
}
