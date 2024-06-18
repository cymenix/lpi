{pkgs}:
with pkgs;
  rustPlatform.buildRustPackage {
    pname = "lpi";
    version = "0.1.0";
    src = ./.;
    cargoSha256 = "sha256-t3j6SL6T08pQw5sE6ZBSjAjOM+9dk4Nw5LGRqnobCWY=";
  }
