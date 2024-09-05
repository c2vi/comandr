{ craneLib
, mkSelString
, mkMizeRustModule
, ...
}:
mkMizeRustModule {
  src = ./.;
  modName = "comandr";
}
