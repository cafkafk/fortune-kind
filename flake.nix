{
  description = "The fortune-kind flake for developing and releasing (soon)";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        buildInputs = with pkgs; lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` & `nix run`:
          default = naersk'.buildPackage {
            src = ./.;
            fortunes = ./fortunes;
            doCheck = true; # run `cargo test` on build
            copyBins = true;
            inherit buildInputs;

            nativeBuildInputs = with pkgs; [makeWrapper];

            preInstall = ''
              installManPage man/fortune-kind.1
              installShellCompletion \
                --bash man/fortune-mod.bash
                --zsh  man/_fortune-mod
                --fish man/fortune-mod.fish
              pwd
              mkdir -p $out
              cp -r $fortunes $out/fortunes;
            '';

            postInstall = ''
              wrapProgram $out/bin/fortune-kind \
                --prefix FORTUNE_DIR : "$out/fortunes"
            '';
          };

          # Run `nix build .#check` to check code
          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
            inherit buildInputs;
          };

          # Run `nix build .#test` to run tests
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit buildInputs;
          };

          # Run `nix build .#clippy` to lint code
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit buildInputs;
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [toolchain just pandoc vhs];
        };

        # for `nix flake check`
        checks = {
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          test = packages.test;
          lint = packages.clippy;
        };
      }
    );
}
