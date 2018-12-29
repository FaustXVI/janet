with import <nixos> {};
stdenv.mkDerivation {
        name = "janet";
        buildInputs = [
          arduino
        ];
    }
