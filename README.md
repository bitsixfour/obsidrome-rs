<img width="1516" height="993" alt="image" src="https://github.com/user-attachments/assets/d49512be-b6d5-483e-bc8f-c9cec3ce004e" />
This is a simple project that creates markdown files from Navidrome's API which are innerconnected between genres and artists. 


##### todo
* cli to deploy it into a vault (trying to get it to be more mature before its "usable")
* Need to add similar artists/movements
* Currently aiming towards indexing monthly listening patterns and use some Git intergration or whatever
* Still Somewhat of a POC: still trying to make it actually useful as a tool instead of just "ooh i need my links." You could probably deploy Quartz with this instead of shilling a last.fm page?
## Use

I wrapped this using a Nix Flake cause my servers use NixOS and also it's easiest lol. 


```nix
{
  inputs.obsidianfm.url = "github:yourname/obsidianfm";

  outputs = { self, nixpkgs, obsidianfm, ... }: {
    nixosConfigurations.server = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        obsidianfm.nixosModules.default
        {
          services.obsidianfm = {
            enable = true;
            createUser = false;
            user = "navidrome";
            group = "navidrome";
            csvPath = "/var/lib/obsidianfm/data.csv";
            vaultRoot = "/srv/obsidianfm-vault";
            pollSeconds = 30;

            navidrome.baseUrl = "http://127.0.0.1:4533";
            navidrome.user = "nix";
            navidrome.password = "your-password";
          };
        }
      ];
    };
  };
}
```


