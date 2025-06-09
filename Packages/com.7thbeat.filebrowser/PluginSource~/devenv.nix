{ pkgs, ... }:

{
  # https://devenv.sh/basics/
  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = "-I\"${pkgs.vulkan-headers}/include\" -I\"${pkgs.xorg.libX11}/include\"";
  env.VULKAN_HEADERS = "${pkgs.vulkan-headers}";

  # https://devenv.sh/packages/
  packages = with pkgs; [ git pkg-config llvmPackages.clang glibc.dev rust-bindgen xorg.libX11 ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
}
