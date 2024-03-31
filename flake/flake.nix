{
  description = "Rust Backend Development Environment With Diesel-cli";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      overlays = [
        rust-overlay.overlays.default
        self.overlay
      ];
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system overlays; });
    in {
      overlay = final: prev: {
        rustToolchain = prev.rust-bin.nightly."2024-02-19".minimal.override {
          extensions = [ "clippy" "rustfmt" "rust-analyzer" "rustc-codegen-cranelift-preview" ];
        };
        rustPlatform = final.makeRustPlatform {
          cargo = final.rustToolchain;
          rustc = final.rustToolchain;
        };
      };

      devShells = forAllSystems (system: { default =
        let
          pkgs = nixpkgsFor.${system};
        in pkgs.mkShell {
          packages = [
            pkgs.rustToolchain
            (pkgs.diesel-cli.override {
              sqliteSupport = false;
              mysqlSupport = false;
            })
          ];
        };
      });
    };
}
