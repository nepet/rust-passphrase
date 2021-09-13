with import <nixpkgs> { };
with pkgs;
stdenv.mkDerivation {
  name = "rust-password-gen";
  buildInputs = [
    figlet
    lolcat
    rustc
    cargo
    libiconv
    openssl
  ];

  shellHook = ''
    figlet "Rust Passwd Gen" | lolcat --freq 0.4
    echo "using rustc: ${rustc}"
    echo "using cargo: ${cargo}"
  '';
}
