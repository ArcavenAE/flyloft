# Cilium Egress Policy Design Notes

## Selector semantics

Cilium egress policies apply to pods matching the `endpointSelector`. When multiple policies match the same endpoint, their rules are **additive** — the union of allowed destinations is permitted. There is no deny-by-default across policies; any policy matching the endpoint contributes its allows.

## DNS-based egress

`toFQDNs` rules let you permit egress by hostname rather than IP. Cilium resolves the DNS at request time and installs ephemeral per-IP allows. This works well for stable CDN-fronted services but can misfire for services that rotate IPs faster than the TTL.

## Wildcard gotchas

`toFQDNs` wildcards like `*.example.com` do NOT match `example.com` itself — only subdomains. To permit both apex and subdomains you must list them separately. This trips up most teams at least once.

## IP block precision

`toCIDRSet` accepts CIDR ranges with `except` exclusions. Prefer narrow ranges; a `/0` allow with excepts is semantically identical to explicit ranges but much harder to audit during incident response.
