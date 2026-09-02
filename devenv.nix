{ pkgs, ... }:

{
  packages = [
    (pkgs.writeShellScriptBin "cargo-ratchet" ''
      exec cargo run --quiet --manifest-path "$DEVENV_ROOT/tools/tdd-ratchet/Cargo.toml" -- "$@"
    '')
    pkgs.actionlint
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
    actionlint
    cargo fmt --check --all
    cargo check -p standards --tests
    cargo ratchet
  '';
}
