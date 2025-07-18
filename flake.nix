{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import (inputs.nixpkgs) { inherit system; });
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ 
            libgcc gtk4 gtk4-layer-shell pkg-config
            cargo rustc rust-analyzer 
            lazygit
          ];
        };
      }
    );
}
