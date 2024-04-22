{
    description = "Bomberman";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay }:
    let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit system overlays;
        };
        rust-toolchain = pkgs.rust-bin.stable.latest.default;
    in {
        devShells.${system}.default = with pkgs; mkShell {
            buildInputs = [
                rust-toolchain
                cargo-watch
                rust-analyzer
                rustfmt
                clippy
                
                pkg-config
                alsa-lib
                libudev-zero
                libxkbcommon
                wayland
                vulkan-loader
                libGL
            ];
            LD_LIBRARY_PATH = "${pkgs.libxkbcommon}/lib:${pkgs.libGL}/lib";
        };
    };
}
