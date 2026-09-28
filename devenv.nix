{ pkgs, ... }:

{
  packages = [
    pkgs.actionlint
    pkgs.gh
    pkgs.git
    pkgs.python3
  ];

  enterTest = ''
    actionlint
  '';
}
