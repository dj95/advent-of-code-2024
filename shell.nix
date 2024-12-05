{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "566e53c2ad750c84f6d31f9ccb9d00f823165550";
    sha256 = "sha256-FLYY5M0rpa5C2QAE3CKLYAM6TwbKicdRK6qNrSHlNrE=";
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
