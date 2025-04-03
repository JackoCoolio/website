{
  config,
  options,
  lib,
  ...
}: let
  challengesRoot = "/var/lib/acme/.challenges";
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
  };

  config = lib.mkIf cfg.enable {
    security.acme = {
      acceptTerms = true;
      defaults.email = cfg.acme.email;
      defaults.server =
        if cfg.acme.useStaging
        then "https://acme-staging-v02.api.letsencrypt.org/directory"
        else "https://acme-v02.api.letsencrypt.org/directory";
      certs.${cfg.acme.domain} = {
        webroot = challengesRoot;
        group = "nginx";
        extraDomainNames = cfg.acme.extraDomains;
      };
    };

    users.users.nginx.extraGroups = ["acme"];

    services.nginx = {
      enable = true;
      virtualHosts = {
        "acmechallenge.${cfg.acme.domain}" = {
          serverAliases = ["*.${cfg.acme.domain}"];
          locations."/.well-known/acme-challenge" = {
            root = challengesRoot;
          };
          locations."/" = {
            return = "301 https://$host$request_uri";
          };
        };
      };
    };

    networking.firewall = {
      enable = true;
      allowedTCPPorts = [80 443];
    };
  };
}
