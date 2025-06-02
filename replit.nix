{ pkgs }:

let
  highway-src = pkgs.fetchFromGitHub {
    owner  = "google";
    repo   = "highway";
    rev    = "1.2.0";
    sha256 = "0ykhc6n3ai18dijdmi38fm1d7pa8i6nbgh64jrxd4499k7jhg568";
  };

  highway = pkgs.stdenv.mkDerivation rec {
    pname = "highway";
    version = "1.2.0";
    src = highway-src;

    nativeBuildInputs = [ pkgs.cmake pkgs.pkg-config ];

    NIX_BUILD_CORES = "2";

    cmakeFlags = [
      "-DHWY_ENABLE_TESTS=OFF"
      "-DHWY_ENABLE_EXAMPLES=OFF"
      "-DHWY_ENABLE_CONTRIB=ON"
      "-DBUILD_SHARED_LIBS=ON"

      # "-DCMAKE_CXX_FLAGS=-DHWY_COMPILE_ONLY_AVX2"

      # tell CMake explicitly where to put things *inside* $out
      "-DCMAKE_INSTALL_INCLUDEDIR=include"
      "-DCMAKE_INSTALL_LIBDIR=lib"
      "-DCMAKE_INSTALL_BINDIR=bin"
    ];
  };
in

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    highway
  ];

  env = {
    CXXFLAGS        = "-I${highway}/include";
    PKG_CONFIG_PATH = "${highway}/lib/pkgconfig";
  };
}
