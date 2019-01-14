with import <nixos> {};
let
  myboost = pkgs.boost.override { enableShared = false; enabledStatic = true; };
in 
stdenv.mkDerivation {
        name = "janet";
        buildInputs = [
          rustup
          arduino
          myboost
          myboost.out
          myboost.dev
          cmake
          lsb-release
          fritzing
        ];
        shellHook = ''
          export PATH="$PATH:${arduino}/share/arduino/hardware/tools/avr/bin"
          export ARDUINO_SDK_PATH="${arduino}/share/arduino/"
          export BOOST_ROOT="${myboost.dev}"
          export BOOST_INCLUDEDIR="${myboost.dev}/include/boost"
          export BOOST_LIBRARYDIR="${myboost.out}/lib"
        '';
    }
