let 
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  arm = import <nixpkgs> { 
    crossSystem = pkgs.lib.systems.examples.raspberryPi; };
in
with pkgs {};
pkgs.stdenv.mkDerivation {
  name = "janet";
  buildInputs = [
    pkgs.arduino
    pkgs.fritzing
    (pkgs.rustChannelOfTargets "nightly" "2018-12-30" [ "x86_64-unknown-linux-gnu" "arm-unknown-linux-gnueabihf" ])
    arm.stdenv.cc
  ];
}
