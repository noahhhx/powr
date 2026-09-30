{ pkgs, lib, ... }:
let
  # winit/wgpu dlopen these at runtime rather than linking them, so they
  # must be findable via LD_LIBRARY_PATH on NixOS.
  runtimeLibs = with pkgs; [
    wayland
    libxkbcommon
    vulkan-loader
    libGL
  ];
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  env.LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibs;

  git-hooks.hooks = {
    clippy.enable = true;
  };
}
