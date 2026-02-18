#!/bin/bash
# Windows Toast Notification Script with Claude Icon
# Usage: ./windows-notify.sh "Title" "Message"

TITLE="${1:-Notification}"
MESSAGE="${2:-Hello from Bash!}"

# Get script directory and resolve icon path
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ICON_PATH="$(cygpath -w "$SCRIPT_DIR/Tray-Win32.ico")"

powershell.exe -ExecutionPolicy Bypass -Command "
Import-Module BurntToast
\$iconPath = '$ICON_PATH'

if (Test-Path \$iconPath) {
    New-BurntToastNotification -Text '$TITLE', '$MESSAGE' -AppLogo \$iconPath
} else {
    Write-Host \"Icon not found: \$iconPath\"
    New-BurntToastNotification -Text '$TITLE', '$MESSAGE'
}
"

echo "Notification sent: $TITLE - $MESSAGE"
