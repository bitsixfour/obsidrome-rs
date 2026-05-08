{ self }:
{ config, lib, pkgs, ... }:

let
  cfg = config.services.obsidianfm;
in
{
  options.services.obsidianfm = {
    enable = lib.mkEnableOption "ObsidianFM scrobbler service";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.system}.default;
      description = "ObsidianFM package to run.";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "obsidianfm";
      description = "User account for the service.";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "obsidianfm";
      description = "Group for the service.";
    };

    createUser = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Whether to create the configured service user and group.";
    };

    csvPath = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/obsidianfm/data.csv";
      description = "Path to the scrobble CSV file.";
    };

    vaultRoot = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/obsidianfm/vault";
      description = "Directory where artist, album, and genre markdown notes are written.";
    };

    pollSeconds = lib.mkOption {
      type = lib.types.ints.positive;
      default = 30;
      description = "Polling interval for Navidrome.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = ''
        Optional environment file for extra runtime settings, such as
        OBSIDIANFM_NAVIDROME_PASSWORD.
      '';
    };

    navidrome = {
      baseUrl = lib.mkOption {
        type = lib.types.str;
        default = "http://127.0.0.1:4533";
        description = "Base URL for the Navidrome server.";
      };

      user = lib.mkOption {
        type = lib.types.str;
        default = "nix";
        description = "Navidrome username.";
      };

      password = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = ''
          Navidrome password exposed directly as plain text through
          OBSIDIANFM_NAVIDROME_PASSWORD.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    users.groups = lib.mkIf cfg.createUser {
      "${cfg.group}" = { };
    };

    users.users = lib.mkIf cfg.createUser {
      "${cfg.user}" = {
        isSystemUser = true;
        group = cfg.group;
        home = "/var/lib/obsidianfm";
        createHome = true;
      };
    };

    systemd.services.obsidianfm = {
      description = "ObsidianFM scrobbler";
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      environment = {
        OBSIDIANFM_CSV_PATH = cfg.csvPath;
        OBSIDIANFM_VAULT_ROOT = cfg.vaultRoot;
        OBSIDIANFM_NAVIDROME_BASE_URL = cfg.navidrome.baseUrl;
        OBSIDIANFM_NAVIDROME_USER = cfg.navidrome.user;
        OBSIDIANFM_NAVIDROME_PASSWORD = cfg.navidrome.password;
        OBSIDIANFM_POLL_SECS = toString cfg.pollSeconds;
      };

      serviceConfig =
        {
          ExecStart = lib.getExe cfg.package;
          User = cfg.user;
          Group = cfg.group;
          Restart = "always";
          RestartSec = "10s";
          StateDirectory = "obsidianfm";
          WorkingDirectory = "/var/lib/obsidianfm";
        }
        // lib.optionalAttrs (cfg.environmentFile != null) {
          EnvironmentFile = [ cfg.environmentFile ];
        }
        ;
    };
  };
}
