{
  description = "fortune-kind: A new kinda fortune.";

  inputs = {
    nixpkgs.url = "http:/rime.cx/v1/github/NixOS/nixpkgs/b/nixpkgs-unstable.tar.gz";

    flake-utils.url = "http://rime.cx/v1/github/semnix/flake-utils.tar.gz";

    naersk.url = "http://rime.cx/v1/github/semnix/naersk.tar.gz";

    rust-overlay.url = "http://rime.cx/v1/github/semnix/rust-overlay.tar.gz";

    treefmt-nix.url = "http://rime.cx/v1/github/semnix/treefmt-nix.tar.gz";
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
            copyLibs = true;
            singleStep = true;
            inherit buildInputs;

            nativeBuildInputs = with pkgs; [makeWrapper installShellFiles];

            MAN_OUT = "./man";

            preBuild = ''
              mkdir -p "./$MAN_OUT";
            '';

            preInstall = ''
              installManPage man/fortune-kind.1
              installShellCompletion \
                --fish man/fortune-kind.fish \
                --bash man/fortune-kind.bash \
                --zsh  man/_fortune-kind
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
          nativeBuildInputs = with pkgs; [rustup toolchain just zip];
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
