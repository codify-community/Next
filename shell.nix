{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = [ 
      binutils-unwrapped rustup 
    ];
}

