{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  packages = [
    pkgs.bacon
    pkgs.cargo
    pkgs.cargo-bump
    pkgs.clippy
    pkgs.rustc
    pkgs.rustfmt
  ];
}
