# https://github.com/rust-rocksdb/rust-rocksdb/issues/537
with import <nixpkgs> { };
pkgs.mkShell {
  buildInputs = with pkgs; [
    clang
    rustup mold git
    man less
    htop bpytop
    wget axel
    valgrind
  ];

  LIBCLANG_PATH = libclang.lib + "/lib/";
}