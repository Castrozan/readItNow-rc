{
  description = "A TUI application for reading and managing notes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "readitnow";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        buildInputs = with pkgs; [
          openssl
          pkg-config
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        # Set environment variables for native TLS
        OPENSSL_NO_VENDOR = 1;

        meta = with pkgs.lib; {
          description = "A TUI application for reading and managing notes";
          homepage = "https://github.com/your-username/readItNow-rc";
          license = licenses.mit; # Update this based on your LICENSE file
          maintainers = [ ];
          platforms = platforms.linux;
        };
      };

      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/readitnow";
      };

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          rustfmt
          openssl
          pkg-config
        ];

        # Set environment variables for development
        OPENSSL_NO_VENDOR = 1;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
}
