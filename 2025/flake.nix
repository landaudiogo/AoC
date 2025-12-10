{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    };
    outputs = { nixpkgs, ... }@inputs:
        let
            system = "x86_64-linux";
            pkgs = nixpkgs.legacyPackages.${system};
        in
        {
            devShells.${system} = {
                default = pkgs.mkShell {
                    packages = with pkgs; [ 
                        cargo 
                        rustc
                        rustfmt
                        rust-analyzer 
                    ] ++ [
                        (pkgs.python3.withPackages (python-pkgs: with python-pkgs; [
                            numpy
                            cvxpy
                            ecos
                        ]))
                    ];
                };
            };
        };
}
