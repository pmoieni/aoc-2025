{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    taplo
    nil

    ((rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override (prev: {
      extensions = prev.extensions ++ [
        "rust-src"
        "rust-analyzer"
      ];
    }))
  ];
}
