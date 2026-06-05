{ pkgs, ... }:

{
  packages = [
    pkgs.cargo
    pkgs.cargo-nextest
    pkgs.clippy
    pkgs.curl
    pkgs.fish
    pkgs.gcc
    pkgs.gh
    pkgs.git
    pkgs.openssl
    pkgs.pkg-config
    pkgs.rustc
    pkgs.rustfmt
    pkgs.tmux
  ];

  enterTest = ''
    cargo fmt --check --all
    cargo check -p standards --tests
    cargo test -p standards
  '';
}
