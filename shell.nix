{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.docker
    pkgs.docker-compose

    pkgs.co.funzzy
  ];
}
