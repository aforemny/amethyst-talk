let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in
with pkgs;
with stdenv;
mkDerivation {
  name = "amethyst-talk";
  buildInputs = [
    latest.rustChannels.stable.rust
    nodejs
    xlibs.libX11
    alsaLib
    pkgconfig
    cmake
    alsaLib
    cmake
    freetype
    latest.rustChannels.stable.rust
    expat
    openssl
    pkgconfig
    python3
    vulkan-validation-layers
    xlibs.libX11
  ];

  APPEND_LIBRARY_PATH = lib.makeLibraryPath [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ];
  shellHook = ''
    export PATH=./node_modules/.bin:$PATH
    export PATH=$HOME/.cargo/bin:$PATH
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
