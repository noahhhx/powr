{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          # winit/wgpu dlopen these at runtime, so bake them into the rpath.
          runtimeLibs = with pkgs; [
            wayland
            libxkbcommon
            vulkan-loader
            libGL
          ];
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "powr";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;

            postFixup = ''
              patchelf --add-rpath ${pkgs.lib.makeLibraryPath runtimeLibs} $out/bin/powr
            '';

            meta.mainProgram = "powr";
          };
        }
      );
    };
}
