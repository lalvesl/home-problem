{
  pkgs ?
    import
      (fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/2d64d17b747edca5055c1ba81da4beec2e71d9a9.tar.gz";
      })
      {
        config = {
          allowUnfree = true;
        };
      },
}:
let
  PROJECT_ROOT = builtins.toString ./.;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # OracleDB dependency
    oracle-instantclient

    # LSPs
    rust-analyzer
    nixd
    typescript
    typescript-language-server

    # Builders / package managers / Languages
    cargo
    llvm
    cmake
    gcc
    pnpm
    rustc
    nodejs_22

    # gRPC protobuf compiler
    protobuf

    # ORM
    sea-orm-cli

    # Testing
    cargo-tarpaulin

    # FMTs
    rustfmt
    nixfmt-rfc-style
    leptosfmt

    # Dev tools
    cargo-watch
    rustfmt
    clippy
    cargo-expand
    bacon
    grpcurl

    # Crawling
    chromedriver
    chromium
    xorg.xvfb

    # Linked libraries
    openssl
    perl
    # zlib
    # musl

    # Some test for use another linker
    # zig

    # CLI utils (improving pure shell)
    jq
    uutils-coreutils-noprefix
    iputils
    pkg-config
    which
    curl
  ];

  # Statically linker does not supported by some libs
  # RUSTFLAGS = "-C target-feature=+crt-static";
  # CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = "${pkgs.musl}/bin/musl-clang";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_WRAPPER = "sccache";
  SCCACHE_DIR = "${PROJECT_ROOT}/target/sccache";
  shellHook = '''';

  env = {
  };
}
