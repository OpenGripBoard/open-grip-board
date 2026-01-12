#!/bin/bash
# Script to generate Mosquitto password file
# Usage: ./scripts/setup-mqtt-passwd.sh <username> <password>

set -e

if [ $# -ne 2 ]; then
    echo "Usage: $0 <username> <password>"
    exit 1
fi

USERNAME=$1
PASSWORD=$2
PASSWD_FILE="configs/passwd"

# Create configs directory if it doesn't exist
mkdir -p configs

# Check if mosquitto_passwd is available
if command -v mosquitto_passwd &> /dev/null; then
    # Create or update password file
    if [ -f "$PASSWD_FILE" ]; then
        mosquitto_passwd -b "$PASSWD_FILE" "$USERNAME" "$PASSWORD"
    else
        mosquitto_passwd -c -b "$PASSWD_FILE" "$USERNAME" "$PASSWORD"
    fi
    echo "Password file created/updated at $PASSWD_FILE"
else
    echo "mosquitto_passwd not found. Install mosquitto-clients or run:"
    echo "docker run -it --rm -v \$(pwd)/configs:/mosquitto/config eclipse-mosquitto mosquitto_passwd -c -b /mosquitto/config/passwd $USERNAME $PASSWORD"
fi
