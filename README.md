# custom dns server
A simple dns server for debugging. 
# Usage
- 1 Download the binary file from the release.
- 2 Config your custom dns rules in the `config.jsom`, the rule will be explained below.
- 3 Make true the binary file and the config file in the same dir. Then `custom-dns-server [--dns true --tcp false --port 53]`.
- 4 By default the udp port 53 is enabled. Feel free to change the port or the portal.
# Simple Rule in json
Have a look at the `config.json`, you'll find it is very simple. An array of item with fields: `name`,`type` and `value`. Until now, the type just support `A` and `CNAME`, because the tool is just for debugging purpose. The name is the domain name. The value is the ipv4 or cname (another domain name).