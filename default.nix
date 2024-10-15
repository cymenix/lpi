{pkgs}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "lpi";
    version = "0.1.1";
    src = ./.;
    cargoHash = "sha256-EwDsrGUt2bHOGkTTtwN8vyJv8PUwgHtTf1fEEH9PF3Y=";
  }
