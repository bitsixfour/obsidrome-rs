<img width="1516" height="993" alt="image" src="https://github.com/user-attachments/assets/d49512be-b6d5-483e-bc8f-c9cec3ce004e" />


## Navidrome/Last.fm -> Obsidian
* Need to add similar artists/movements
* Currently aiming towards indexing monthly listening patterns (picrel is monthly)
* Still Somewhat of a POC: still trying to make it actually useful as a tool instead of just "ooh i need my links." You could probably deploy Quartz with this instead of shilling a last.fm page?

##### todo
* cli to deploy it into a vault (trying to get it to be more mature before its "usable")
* Access MusicBrainz for genres (they're good enough imo)

## Flake

This repo now exposes:

* `packages.<system>.default`
* `apps.<system>.default`
* `devShells.<system>.default`
* `nixosModules.default`

Example NixOS usage:

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

Runtime configuration is exposed through environment variables as well:

* `OBSIDIANFM_CSV_PATH`
* `OBSIDIANFM_VAULT_ROOT`
* `OBSIDIANFM_NAVIDROME_BASE_URL`
* `OBSIDIANFM_NAVIDROME_USER`
* `OBSIDIANFM_NAVIDROME_PASSWORD`
* `OBSIDIANFM_POLL_SECS`

The service user needs write access to both the CSV path and the vault root.
