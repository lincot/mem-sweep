let
  pkgs = import <nixpkgs> {};
in
import ./replit.nix { inherit pkgs; }
