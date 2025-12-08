{
  inputs = {
    systems.url = "github:nix-systems/default";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      systems,
      nixpkgs,
      rust-overlay,
    }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          let
            fArgs = builtins.functionArgs f;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
          in
          if fArgs == { } then (f pkgs) else (f { inherit system pkgs; })
        );
    in
    {
      devShells = forEachSystem (pkgs: {
        default = import ./shell.nix {
          inherit pkgs;
        };
      });
    };
}
