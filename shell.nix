{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "2c27ab2e60502d1ebb7cf38909de38663f762a79";
    sha256 = "sha256-n/DOfpKH1vkukuBnach91QBQId2dr5tkE7/7UrkV2zw=";
  };

  pkgs = import pinnedPkgs {};

  inherit (lib) optional optionals;
  inherit (darwin.apple_sdk.frameworks) Cocoa CoreGraphics Foundation IOKit Kernel OpenGL Security;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-audit
    cargo-edit
    cargo-watch
    cargo-nextest
    rustfmt
    clippy
    libiconv
    rustc
    openssl
    pkg-config
  ] ++ optionals stdenv.isDarwin [
    Cocoa
    CoreGraphics
    Foundation
    IOKit
    Kernel
    OpenGL
    Security
    libpng
    zlib
  ];

  RUSTONIG_DYNAMIC_LIBONIG = "1";
}
