{
  description = "Distributex - HTMX + NGINX + NIX";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    conipkgs.url = "github:cristianoliveira/nixpkgs";
  };
  outputs = { self, nixpkgs, utils, conipkgs }: 
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { 
          inherit system;
          overlays = [ (final: prev: { 
            co = import conipkgs { pkgs = final; };
          })];
        };
      in {
        devShells.default = import ./shell.nix { inherit pkgs; };
    });
}
