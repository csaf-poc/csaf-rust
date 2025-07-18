#!/usr/bin/env bash

rsync_url() {
    if [ $# -ne 2 ]; then
        echo "Usage: rsync_url <URL> <destination>" >&2
        return 2
    fi

    url="$1"
    dest="$2"

    # Create a temporary file
    tmp=$(mktemp) || {
        echo "Error: mktemp failed" >&2
        return 1
    }

    # Ensure the temporary file is removed on function exit
    trap 'rm -f "$tmp"' RETURN

    # Download the URL to the temporary file using curl or wget
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$tmp" || {
            echo "Error: download via curl failed" >&2
            return 1
        }
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$tmp" "$url" || {
            echo "Error: download via wget failed" >&2
            return 1
        }
    else
        echo "Error: neither curl nor wget is installed." >&2
        return 1
    fi

    # Sync the temporary file to the destination with rsync and checksum
    rsync -c "$tmp" "$dest" || {
        echo "Error: rsync failed" >&2
        return 1
    }

    return 0
}

rsync_url https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry csaf-rs/assets/language-subtag-registry.txt
