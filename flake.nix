{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; rec {
        devShell = mkShell rec {
          buildInputs = [
            libxkbcommon
            libGL

            # WINIT_UNIX_BACKEND=wayland
            wayland

            # WINIT_UNIX_BACKEND=x11
            xorg.libXrandr
            xorg.libXi
            xorg.libX11

            pkg-config
            alsa-lib
            openssl
            vulkan-loader       # If using Vulkan
            pulseaudio         # If your app uses PulseAudio
            SDL2               # Common dependency for graphics
            libudev-zero
            xorg.libXcursor
          ];
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      });
}
