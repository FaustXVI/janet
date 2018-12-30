with import <nixos> {};
stdenv.mkDerivation {
        name = "janet";
        buildInputs = [
          arduino
          cmake
          lsb-release
          fritzing
        ];
        shellHook = ''
          export PATH="$PATH:${arduino}/share/arduino/hardware/tools/avr/bin"
          export ARDUINO_SDK_PATH="${arduino}/share/arduino/"
        '';
    }
