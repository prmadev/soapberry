{
  description = "Soapberry";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = ["x86_64-linux"]; # "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"

      perSystem = {
        # config,
        # self',
        # inputs',
        pkgs,
        # system,
        ...
      }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        devenv.shells.default = let
          name = "Soapberry";
        in {
          inherit name;

          # environmental variables
          env = {
          };

          languages = {
            nix.enable = true;
            rust = {
              enable = true;
              channel = "stable";
              components = ["rustc" "clippy" "rustfmt" "rust-analyzer" "rust-src"];
            };
          };

          # https://devenv.sh/reference/options/
          # packages = let
          #   rust-support-packages = with pkgs; [
          #   ];
          # in
          #   rust-support-packages;

          pre-commit = {
            hooks = {
              # for nix
              deadnix.enable = true;
              alejandra.enable = true;
              nil.enable = true;

              # for markdown
              markdownlint.enable = true;

              # for github
              actionlint.enable = true;

              # for git
              commitizen.enable = true;

              # for rust
              cargo-check.enable = true;
              clippy.enable = true;
              rustfmt.enable = true;
              typos.enable = true;
              taplo.enable = true;
            };
            settings = {
              clippy = {
                allFeatures = true;
                denyWarnings = true;
              };
            };
          };

          scripts = {
            tr.exec = "exa --tree  --git-ignore --icons --group-directories-first  --all  --long --git --no-permissions --no-user --no-filesize --no-time";
          };

          services = {
          };
        };
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}