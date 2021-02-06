with import ./pkgs.nix {};

stdenv.mkDerivation rec {
  name = "rts-simple-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    rust-analyzer
    SDL2
    SDL2_mixer
    valgrind
    kdeApplications.kcachegrind
    llvm_11
  ];
}
