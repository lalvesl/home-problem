{
  pkgs ?
    import
      (fetchTarball {
        # Replace for 25.05 when released
        url = "https://github.com/NixOS/nixpkgs/archive/2d64d17b747edca5055c1ba81da4beec2e71d9a9.tar.gz";
      })

      # <nixpkgs>
      {
        config = {
          allowUnfree = true;
        };
      },
}:

let
  getEnvDefault =
    name: default:
    let
      val = builtins.getEnv name;
    in
    if val == "" then default else val;
  port = getEnvDefault "SERVER_PORT" "9876";
  grpc_port = getEnvDefault "GRPC_PORT" "9877";
  image_name = getEnvDefault "IMAGE_RELEASE_NAME" "cdp";
  app_name = getEnvDefault "PR_NM" "wmosales-cdp";
  smb_mount_point = getEnvDefault "SMB_MOUNT_POINT" "/app/EP0";
  buildImage = pkgs.dockerTools.buildImage;
  mkDerivation = pkgs.stdenv.mkDerivation;
  app_folder = mkDerivation {
    name = "app-folder";
    src = ./.;
    installPhase = ''
      mkdir -p $out
    '';
    buildPhase = ''
      mkdir -p $out/app
      mkdir -p $out${smb_mount_point}
      cp ./target/release/${app_name} $out/app
      mkdir -p $out/tmp
      chmod 777 -R $out/tmp
    '';
  };
in
buildImage {
  name = image_name;
  tag = "latest";

  copyToRoot = pkgs.buildEnv {
    name = "image-root";
    pathsToLink = [
      "/bin"
      "/app"
      "/etc/ssl/certs"
      "/tmp"
    ];
    paths = with pkgs; [
      app_folder

      # oracle db integration
      oracle-instantclient

      # TLS and SSL conections
      openssl
      cacert

      # Debug tools
      bash
      coreutils
      iputils

      which

      chromedriver
      chromium
      xorg.xvfb

      # Linker debug
      # glibc
    ];
  };

  config = {
    Env = [
      # Force SSL certificates
      "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"

      # Expose oracle db integration
      "LD_LIBRARY_PATH=${
        pkgs.lib.makeLibraryPath [
          pkgs.oracle-instantclient
          pkgs.openssl
        ]
      }"
    ];

    WorkingDir = "/app";

    Cmd = [
      "/app/${app_name}"
    ];

    ExposedPorts = {
      "${port}/tcp" = { };
      "${grpc_port}/tcp" = { };
    };
  };
}
