{
    description = "A Nix-flake-based development environment for Auto Peer Client";

    inputs = {
        nixpkgs.url      = "github:NixOS/nixpkgs/nixos-23.11";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay, ... }: let
        system = "x86_64-linux";
    in {
        devShells."${system}" = {
            default = let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {
                    inherit system overlays;
                };
            in pkgs.mkShell {
                packages = with pkgs; [
                    openssl
                    pkg-config
                    protobuf
                    rust-bin.stable.latest.default
                    rust-analyzer
                    clang-tools
                ];

                shellHook = ''
                    exec fish
                '';
            };
        };
    };
}
