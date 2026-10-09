{ pkgs ? import <nixpkgs> { } }:

let
  # miniquad dlopens these at runtime, so they must be on LD_LIBRARY_PATH
  runtimeLibs = with pkgs; [
    libx11
    libxi
    libxcursor
    libxrandr
    libGL
    libxkbcommon
    wayland
    alsa-lib
  ];
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ cargo rustc pkg-config cargo-watch ];
  buildInputs = runtimeLibs;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
  # the nix rustc ships no lib/rustlib/src, so rust-analyzer needs the std source pointed out
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
}
