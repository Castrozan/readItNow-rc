{ pkgs, ... }:

{
  cachix.enable = false;

  # Project name
  name = "readitnow";

  # Enable Rust language support
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # Add required system packages
  packages = with pkgs; [
    # Build essentials
    gcc
    pkg-config

    # For image processing (image crate dependency)
    libjpeg
    libpng

    # For potential SSL/TLS support (reqwest dependency)
    openssl

    # Development tools
    rust-analyzer
    rustfmt
    clippy
  ];

  # Scripts for common development tasks
  scripts = {
    # Build the project
    "build" = {
      exec = ''
        cargo build "$@"
      '';
    };

    # Run the ReadItNow application
    "readitnow" = {
      exec = ''
        cargo run "$@"
      '';
    };

    # Development mode - runs with debug flags and full output
    "readitnow-dev" = {
      exec = ''
        RUST_LOG=debug cargo run "$@"
      '';
    };

    # Run tests
    "test" = {
      exec = ''
        cargo test "$@"
      '';
    };

    # Format code
    "fmt" = {
      exec = ''
        cargo fmt "$@"
      '';
    };

    # Lint code
    "lint" = {
      exec = ''
        cargo clippy "$@"
      '';
    };
  };

  # Environment variables
  env = {
    # Rust log level for development
    RUST_LOG = "info";

    # Ensure pkg-config can find system libraries
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.libjpeg.dev}/lib/pkgconfig";
  };
}
