#!/usr/bin/env bash
set -euo pipefail

# Get security group ID
security_group_id="$(
  aws ec2 describe-instances |
    jq -r '.Reservations[0].Instances[0].SecurityGroups[0].GroupId'
)"

# Delete existing ingress rules
rule_ids=$(
  aws ec2 describe-security-group-rules \
    --filters "Name=group-id,Values=$security_group_id" |
    jq -r '.SecurityGroupRules[] | select(.IsEgress==false) | .SecurityGroupRuleId'
)
if [[ $rule_ids ]]
then
  aws ec2 revoke-security-group-ingress \
    --group-id "$security_group_id" \
    --security-group-rule-ids $rule_ids \
    >/dev/null
fi

# Add laptop IP
my_ip="$(curl --silent https://ipinfo.io/ip)"
today="$(date +'%Y/%m/%d')"

tcp_permissions_vals=(
  'IpProtocol=tcp'
  'FromPort=0'
  'ToPort=65535'
  "IpRanges=[{CidrIp=${my_ip}/32,Description='${today} Peter Laptop'}]"
)
tcp_permissions=$(IFS=,; echo "${tcp_permissions_vals[*]}")

udp_permissions_vals=(
  'IpProtocol=udp'
  'FromPort=0'
  'ToPort=65535'
  "IpRanges=[{CidrIp=${my_ip}/32,Description='${today} Peter Laptop'}]"
)
udp_permissions=$(IFS=,; echo "${udp_permissions_vals[*]}")

aws ec2 authorize-security-group-ingress \
  --group-id "$security_group_id" \
  --ip-permissions "${tcp_permissions}" \
  >/dev/null
>&2 echo "Created TCP permissions for ${my_ip}"
aws ec2 authorize-security-group-ingress \
  --group-id "$security_group_id" \
  --ip-permissions "${udp_permissions}" \
  >/dev/null
>&2 echo "Created UDP permissions for ${my_ip}"
  >/dev/null
