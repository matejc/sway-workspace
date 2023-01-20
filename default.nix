{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  build = rustPlatform.buildRustPackage {
    name = "sway-workspace";

    src = ./.;

    cargoSha256 = "";
  };

  shell = mkShell {
    nativeBuildInputs = with pkgs; [ rustc cargo ];
    RUST_BACKTRACE = 1;
  };
in {
  inherit build shell;
}
