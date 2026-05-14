
This is a simple project that creates markdown files from Navidrome's API which are innerconnected between genres and artists. You could push this to Quartz to have a cool mind map or whatever.

My website's example -> https://obsidian.wngyn.net

##### todo
* cli to deploy it into a vault (trying to get it to be more mature before its "usable")
* Need to add similar artists/movements
* Currently aiming towards indexing monthly listening patterns and use some Git intergration or whatever
* Still Somewhat of a POC: still trying to make it actually useful as a tool instead of just "ooh i need my links." You could probably deploy Quartz with this instead of shilling a last.fm page?
## Use

I wrapped this using a Nix Flake cause my servers use NixOS and also it's easiest. 


```nix
{
  inputs.obsidianfm.url = "github:bitsixfour/obsidrome-rs;

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


