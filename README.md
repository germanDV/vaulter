# Vaulter

Vaulter is a CLI tool for managing secrets in a local database with encryption at rest.

Getting a secret from the vault copies it to the clipboard.

Recommended clipboards:
- Wayland: **wl-copy**
- X11: **xsel** _(xclip is not supported)_

## Installation

Downaload a binary for your platform. (**NOT available yet**)

## Usage

```bash
vaulter --help
```

## Config

Environment variables:
- `VAULTER_PASSPHRASE` - The passphrase used to encrypt secrets. If not provided, you will be prompted to enter it.
- `VAULTER_DB_PATH` - The path to the database file. If not provided, it will default to `<your-local-data-dir>/vaulter/vault.db`, for example on Linux `~/.local/share/vaulter/vault.db`.

