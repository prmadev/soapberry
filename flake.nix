{
  description = "Soapberry";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
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

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
    advisory-db,
    fenix,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      commonArgs = {
        inherit src;
        inherit cargoArtifacts;
        inherit name;
        inherit version;
        inherit pname;

        buildInputs =
          [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

        # Additional environment variables can be set directly
        # MY_CUSTOM_VAR = "some value";
      };

      rustWithMuslTarget = pkgs.rust-bin.stable.latest.default.override {
        targets = ["x86_64-unknown-linux-musl"];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustWithMuslTarget;

      kyushu_items = craneLib.crateNameFromCargoToml {cargoToml = ./crates/kyushu/Cargo.toml;};
      src = craneLib.cleanCargoSource (craneLib.path ./.);
      name = kyushu_items.pname;
      version = kyushu_items.version;
      pname = kyushu_items.pname;

      kyushu = craneLib.buildPackage (commonArgs
        // {
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          doCheck = true;
        });
    in {
      checks =
        {
          inherit kyushu;

          the-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          the-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          the-fmt =
            craneLib.cargoFmt commonArgs
            // {
              inherit src;
            };

          the-audit = craneLib.cargoAudit (commonArgs
            // {
              inherit src advisory-db;
            });

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          the-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        }
        # // lib.optionalAttrs (system == "x86_64-linux") {
        # NB: cargo-tarpaulin only supports x86_64 systems
        # Check code coverage (note: this will not upload coverage anywhere)
        # my-crate-coverage = craneLib.cargoTarpaulin (commonArgs
        # // {
        # inherit cargoArtifacts;
        # });
        # }
        ;

      packages.kyushu = kyushu;
      packages.default = kyushu;

      apps.default = flake-utils.lib.mkApp {
        drv = kyushu;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks.${system};

        # Additional dev-shell environment variables can be set directly
        # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
        ];
      };
    });
}
