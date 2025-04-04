{
  config,
  lib,
  ...
}: let
  cfg = config.wambolt.website;
in {
  options.wambolt.website = {
    enable = lib.mkOption {
      type = lib.types.bool;
      description = ''
        Whether to run the website.
      '';
      default = false;
      example = true;
    };
    acme = lib.mkOption {
      description = ''
        Information given to the ACME client that handles SSL certs.
      '';
      type = lib.types.submodule {
        options = {
          email = lib.mkOption {
            type = lib.types.str;
            example = "admin+acme@example.com";
            description = ''
              Contact email for Let's Encrypt.
            '';
          };
          domain = lib.mkOption {
            type = lib.types.str;
            example = "example.com";
            description = ''
              The domain to retrieve certs for.
            '';
          };
          extraDomains = lib.mkOption {
            type = lib.types.listOf lib.types.str;
            description = ''
              Other domains to retrieve certs for.
            '';
            default = [];
            example = ["subdomain.example.com"];
          };
          useStaging = lib.mkOption {
            type = lib.types.bool;
            description = ''
              Whether to use Let's Encrypt's staging endpoint instead of the
              production one.
            '';
            default = false;
            example = true;
          };
        };
      };
    };
    package = lib.mkOption {
      type = lib.types.package;
      description = ''
        The package to use for the server.
      '';
    };
    bind = lib.mkOption {
      type = lib.types.str;
      description = ''
        The address to bind to.
      '';
      default = "0.0.0.0";
    };
    port = lib.mkOption {
      type = lib.types.int;
      description = ''
        The port to bind HTTP to.
      '';
      example = 80;
    };
  };

  config = lib.mkIf cfg.enable {
    security.acme = {
      acceptTerms = true;
      defaults.email = cfg.acme.email;
      defaults.server =
        if cfg.acme.useStaging
        then "https://acme-staging-v02.api.letsencrypt.org/directory"
        else "https://acme-v02.api.letsencrypt.org/directory";
    };

    services.nginx = {
      enable = true;

      recommendedProxySettings = true;
      recommendedTlsSettings = true;

      virtualHosts = let
        base = {
          forceSSL = true;
          locations."/" = {
            proxyPass = "http://127.0.0.1:${builtins.toString cfg.port}";
          };
        };
      in
        {
          "${cfg.acme.domain}" =
            {
              enableACME = true;
              serverAliases = cfg.acme.extraDomains;
            }
            // base;
        }
        // lib.genAttrs cfg.acme.extraDomains (
          domain:
            {
              useACMEHost = cfg.acme.domain;
            }
            // base
        );
    };

    networking.firewall.allowedTCPPorts = [80 443];

    systemd.services.website = {
      enable = true;

      description = "${cfg.acme.domain} web server";
      wantedBy = ["multi-user.target"];
      before = ["nginx.service"];

      stopIfChanged = false;
      startLimitIntervalSec = 60;

      serviceConfig = {
        # TODO: may be better to let user (me) configure
        User = "nginx";
        Group = "nginx";

        ExecStart = ''
          ${cfg.package}/bin/website \
            --host ${cfg.bind} \
            --port ${builtins.toString cfg.port}
        '';

        Restart = "always";
        RestartSec = "10s";

        WorkingDirectory = "${cfg.package}/";

        # TODO: maybe cap_sys_resource?
        AmbientCapabilities = ["CAP_NET_BIND_SERVICE"];

        ProcSubset = "pid";
        ProtectProc = "invisible";

        UMask = "0027";

        NoNewPrivileges = true;

        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectHostname = true;
        ProtectClock = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = ["AF_UNIX" "AF_INET" "AF_INET6"];
        RestrictNamespaces = true;
        LockPersonality = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        RemoveIPC = true;
        PrivateMounts = true;

        SystemCallArchitectures = "native";
        SystemCallFilter = ["~@cpu-emulation @debug @keyring @mount @obsolete @privileged @setuid"];
      };
    };
  };
}
