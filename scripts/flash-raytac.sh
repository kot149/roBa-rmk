#!/bin/bash

set -u

if [ -z "${1:-}" ]; then
    echo "Usage: $0 <path_to_uf2_file>" >&2
    exit 1
fi

UF2_FILE="$1"
BOARD_ID_PATTERN='^Board-ID:[[:space:]]+nRF52840-MDBT50Q_RX-verD[[:space:]]*$'
BOOTLOADER_PATTERN='^UF2 Bootloader[[:space:]]+0\.9\.2([[:space:]]|$)'
SOFTDEVICE_PATTERN='^SoftDevice:[[:space:]]+S140[[:space:]]+6\.1\.1([[:space:]]|$)'

if [ ! -f "$UF2_FILE" ]; then
    echo "Error: File '$UF2_FILE' not found." >&2
    exit 1
fi

echo "Firmware file: $UF2_FILE"
echo "Required INFO_UF2.TXT metadata: Board-ID nRF52840-MDBT50Q_RX-verD, UF2 Bootloader 0.9.2, SoftDevice S140 6.1.1"

is_raytac_loader() {
    local mount_point="$1"
    local info_file="${mount_point}/INFO_UF2.TXT"

    [ -d "$mount_point" ] && [ -f "$info_file" ] || return 1
    grep -Eiq "$BOARD_ID_PATTERN" "$info_file" || return 1
    grep -Eiq "$BOOTLOADER_PATTERN" "$info_file" || return 1
    grep -Eiq "$SOFTDEVICE_PATTERN" "$info_file" || return 1
}

write_firmware() {
    local target_mount_point="$1"
    local source_file="$2"

    if ! is_raytac_loader "$target_mount_point"; then
        echo "Error: '$target_mount_point' no longer reports the required Raytac UF2 metadata." >&2
        exit 1
    fi

    echo "Copying Raytac firmware to \"${target_mount_point}\"..."
    COPYFILE_DISABLE=1 cp "$source_file" "${target_mount_point}/" || {
        echo "Error: Failed to copy firmware." >&2
        exit 1
    }
    echo "Flash completed!"
    sleep 2
}

trap 'echo -e "\nCancelled by user."; exit 0' INT

echo "Checking mounted drives for a compatible Raytac UF2 loader..."
for drive_path in /Volumes/*; do
    [ -e "$drive_path" ] || continue
    if is_raytac_loader "$drive_path"; then
        echo "Compatible Raytac UF2 loader found at \"$drive_path\""
        write_firmware "$drive_path" "$UF2_FILE"
        exit 0
    fi
done

echo "No compatible Raytac UF2 loader found."
echo "Waiting for a Raytac UF2 loader drive... (Press 'q' to cancel)"
before_drives_str=$(find /Volumes -maxdepth 1 -mindepth 1 -exec basename {} \;)

while true; do
    if read -t 1 -n 1 key 2>/dev/null; then
        if [[ "$key" == "q" || "$key" == "Q" ]]; then
            echo -e "\nCancelled by user."
            exit 0
        fi
    fi

    after_drives_str=$(find /Volumes -maxdepth 1 -mindepth 1 -exec basename {} \;)
    new_drive_names=$(comm -13 <(echo "$before_drives_str" | sort) <(echo "$after_drives_str" | sort))

    if [ -n "$new_drive_names" ]; then
        while IFS= read -r drive; do
            [ -z "$drive" ] && continue
            echo "New drive detected: \"$drive\""
            mount_point="/Volumes/${drive}"
            sleep 1

            if is_raytac_loader "$mount_point"; then
                echo "Compatible Raytac UF2 loader detected at \"$mount_point\""
                write_firmware "$mount_point" "$UF2_FILE"
                exit 0
            fi
            echo "Drive \"$drive\" does not report the required Raytac metadata, skipping..."
        done <<< "$new_drive_names"
    fi
    before_drives_str=$after_drives_str
done
