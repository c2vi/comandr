{
  description = "comandr flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        packages = rec {
          hello = pkgs.hello;
          default = hello;
        };
        apps = rec {
          hello = flake-utils.lib.mkApp { drv = self.packages.${system}.hello; };
          default = hello;
        };

      devShells.default = pkgs.mkShell {

        nativeBuildInputs = with pkgs; [
          cargo-generate
          pkg-config 
          patchelf
          libsForQt5.full
          cmake
          #lldb gdb
        ];

        buildInputs = with pkgs; [
        ];

        shellHook = ''
          echo hiiiiiiiiiiiiii
          export LD_LIBRARY_PATH=${pkgs.webkitgtk_4_1}/lib:${pkgs.libsoup_3}/lib:${pkgs.glib.out}/lib:${pkgs.gtk3}/lib:${pkgs.cairo}/lib:${pkgs.gdk-pixbuf}/lib:${pkgs.libxkbcommon}/lib:${pkgs.fontconfig.lib}/lib:${pkgs.libsForQt5.full}/lib:${pkgs.stdenv.cc.cc.lib}/lib
          export CPLUS_INCLUDE_PATH=${pkgs.libsForQt5.full}/include:${pkgs.libGL.dev}/include
          export MME_QT_LIB=${pkgs.libsForQt5.full}/lib

          # i found that this is the env war to set where QT looks for platform plugins
          # at: https://forums.fedoraforum.org/showthread.php?326508-How-to-set-QT_QPA_PLATFORM_PLUGIN_PATH
          export QT_QPA_PLATFORM_PLUGIN_PATH=${pkgs.libsForQt5.full}/lib/qt-5.15.14/plugins/platforms/
        '';

      };
    }
  );
}
