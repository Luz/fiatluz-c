{
  description = "dev env";
  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
#stdenv.mkDerivation rec {
    devShells.${system}.default = (pkgs.buildFHSEnv {
      name = "cuda-env";
      targetPkgs = pkgs: (with pkgs; [
        valgrind
      ]);
    }).env;
  };
}

