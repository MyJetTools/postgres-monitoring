### Settings file example


```yaml
envs:
  env1-name:
    db-instance-1: 'Server=10.1.0.3;Database=postgres;Port=5432;User Id=admin;Password=xxx;ssh=root@10.0.0.0:22'
    db-instance-2: 'Server=10.1.0.4;Database=postgres;Port=5432;User Id=admin;Password=zzzz;ssh=root@10.0.0.3:22'

  env2-name: 
    db-instance-1: 'Server=10.0.0.2;Database=postgres;Port=5432;User Id=admin;Password=xxxx;ssh=root@11.0.0.0:22'
    db-instance-2: 'Server=10.0.0.3;Database=postgres;Port=5432;User Id=admin;Password=xxxx;ssh=root@11.0.0.1:22'

ssh_credentials:
  "root@11.0.0.1:22":
    cert_path: /cert1
    cert_pass_prase: passphrase-for-cert-1
  "*":
    cert_path: /cert2
    cert_pass_prase: passphrase-for-cert-2
```


* ssh-credentials - is an optional field. If it is not provided, ssh agent will be used to authenticate.