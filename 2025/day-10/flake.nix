{
  description = "Rust + z3 dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux.default = let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        pkg-config
        clang
        llvmPackages.libclang
        rustc
        cargo
        z3
      ];

      # This tells bindgen where to find libclang.so
      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      # Needed in some cases
      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";
    };
  };
}

