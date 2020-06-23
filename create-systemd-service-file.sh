#!/bin/sh

SYSTEMD_FILE="sometimes-red-sometimes-blue-discord-bot.service"
USER="nobody"

if [ $# -gt 1 ]; then
    DISCORD_TOKEN="$1"
    EXECUTABLE_PATH="$2"
else
    echo "$(basename -- "$0") discord-token executable-path [user]" >&2
    exit 1
fi

cat > "$SYSTEMD_FILE" << EOF
[Unit]
Description=A Discord bot that replies red or blue whenever someone speaks to it
After=network.target

[Service]
Type=simple
User=${USER}
WorkingDirectory=/tmp/
ExecStart=${EXECUTABLE_PATH}
Restart=always
Environment="DISCORD_TOKEN=${DISCORD_TOKEN}"

[Install]
WantedBy=multi-user.target
EOF
