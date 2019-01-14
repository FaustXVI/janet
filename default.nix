with import <nixos> {};
stdenv.mkDerivation {
        name = "janet";
        buildInputs = [
          rustup
          arduino
          fritzing
        ];
    }
