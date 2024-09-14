#!/bin/sh
#sh api_get.sh 8e8cf95ab05611ec925db827eba5f2ca 192.168.1.25 hendriksen-mark
#   $0         $1				$2	     $3

# Fail on errors + undefined variables
set -ue

IP=$2
KEY=$1

echo "Dumping v2 state.."
curl -s -k -H"hue-application-key: $KEY" https://$IP/clip/v2/resource > hue-v2-sample-$(date -I)-$3.json

echo "Dumping v1 state.."
curl -s -k https://$IP/api/$KEY/ > hue-v1-sample-$(date -I)-$3.json

echo "Done"
