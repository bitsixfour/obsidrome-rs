let
  pkgs = import <nixpkgs> { };

  libraries = with pkgs; [
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    glib-networking
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    openssl
    pango
    webkitgtk_4_1
  ];
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    bun
    cargo
    cargo-tauri
    gobject-introspection
    nodejs
    pkg-config
    rustc
    wrapGAppsHook3
  ];

  buildInputs = libraries ++ (with pkgs; [
    gsettings-desktop-schemas
  ]);

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
    export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
  '';
}
