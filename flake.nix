{
  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      appName = "mist_bar";

      naersk' = pkgs.callPackage naersk { };
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [ 
          libgcc gtk4 gtk4-layer-shell pkg-config
          cargo rustc rust-analyzer 
          lazygit
        ];
      };

      packages.${system} = {
        ${appName} = naersk'.buildPackage {
          pname = "${appName}";
          version = "0.1.0";
          src = ./.;
          buildInputs = with pkgs; [ 
            libgcc gtk4 gtk4-layer-shell pkg-config
            cargo rustc rust-analyzer 
            lazygit
          ];
        };
      };

      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.${appName}}/bin/${appName}";
      };
    };
}
