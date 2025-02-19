{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, ... } @ inputs: inputs.flake-utils.lib.eachDefaultSystem (system:
    let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.default = with pkgs; mkShell {
        buildInputs = [
          rust-bin.nightly.latest.default
        ];
      };
    }
  );
}

