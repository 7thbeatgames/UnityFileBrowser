{ pkgs, lib, ... }:

{
  # https://devenv.sh/basics/
  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  # env.BINDGEN_EXTRA_CLANG_ARGS = "-I\"${pkgs.vulkan-headers}/include\" -I\"${pkgs.xorg.libX11}/include\" -I\"${pkgs.libGL.dev}/include\" -I\"${pkgs.gtk3.dev}/include/gtk-3.0\" -I\"${pkgs.glib.dev}/include/glib-2.0\"";

  # https://devenv.sh/packages/
  packages = with pkgs; [ git pkg-config llvmPackages.clang glibc.dev rust-bindgen xorg.libX11 glib.dev libGL.dev gtk3.dev vulkan-headers ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
}
