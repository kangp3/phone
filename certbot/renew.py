#!/usr/bin/env python3
# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "cloudflare",
# ]
# ///
import os

from cloudflare import Cloudflare


FRANDLINE_ZONE = "frandline.com"
DNS_RECORD = "pbx.frandline.com"
CLOUDFLARE_KEY = REDACTED
CERTBOT_VALIDATION_TOKEN = os.environ["CERTBOT_VALIDATION"]

client = Cloudflare(api_token=CLOUDFLARE_KEY)

frandline_zone = None
for zone in client.zones.list():
    if zone.name == FRANDLINE_ZONE:
        frandline_zone = zone
assert frandline_zone is not None, f'No zone with name "{FRANDLINE_ZONE}" found'

dns_records = client.dns.records.list(zone_id=frandline_zone.id)

acme_record = None
record_name = f"_acme-challenge.{DNS_RECORD}"
for record in dns_records:
    if record.type == "TXT" and record.name == record_name:
        acme_record = record
        break
assert acme_record is not None, f'No DNS record with name "{record_name}" found'

resp = client.dns.records.edit(
    acme_record.id,
    zone_id=frandline_zone.id,
    type=acme_record.type,
    name=acme_record.name,
    content=CERTBOT_VALIDATION_TOKEN,
)
print(f"Updated {acme_record.name} with content {CERTBOT_VALIDATION_TOKEN[:5]}...{CERTBOT_VALIDATION_TOKEN[-5:]}")
