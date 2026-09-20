![bage logo](./assets/bage.svg)

Bage (Bulk Age) is a command-line tool designed to perform bulk encryption using the age encryption format. It allows you to encrypt multiple folders in a single operation, with support for selective inclusion, exclusion, cleanup, and filename randomization.

## Requirements

Before using bage, ensure that the following are available on your system:

- A Unix-like environment (Linux or macOS)
- `curl` for installation

## Installation

Install bage by running the following command in your terminal:

```bash
curl -fsFL https://raw.githubusercontent.com/hwisnu222/bage/main/install.sh
```

The install script will download the bage binary and place it in your system path.

## Usage

Bage provides two primary subcommands:

- `encrypt` for encrypting folders
- `decrypt` for decrypting encrypted content

### Encrypt

#### Basic Encryption

To encrypt all folders in the current working directory:

```bash
bage encrypt
```

This command scans the current directory and encrypts every folder it finds.

#### Encrypting Folders in a Specific Path

If your target folders are located in a different directory, specify the path with the `-p` or `--path` flag:

```bash
bage encrypt -p dir/
```

All subfolders inside `dir` will be encrypted.

#### Encrypting Specific Folders

To encrypt only selected folders, use the `-i` or `--include` flag followed by a comma-separated list of folder names:

```bash
bage encrypt -i dir,dir1,dir2
```

#### Excluding Folders from Encryption

To encrypt all folders except certain ones, use the `-e` or `--exclude` flag:

```bash
bage encrypt -p dir -e subdir,subdir1
```

#### Cleaning Up After Encryption

To delete the original folders once encryption is complete, add the `--clean` flag:

```bash
bage encrypt -p dir -i subdir,subdir1,subdir2 --clean
```

This is useful for removing plaintext source data after it has been securely encrypted.

#### Randomizing Filenames

To replace original filenames with randomized names during encryption, use the `-x` flag:

```bash
bage encrypt -x
```

This adds an extra layer of privacy by obscuring the identity of the encrypted content.

#### Dry Run

To preview which folders would be processed without actually encrypting them, use the `-d` flag:

```bash
bage encrypt -d
```

This is helpful for verifying your command before committing to the operation.

### Decrypt

The `decrypt` subcommand reverses the encryption process. It supports a limited subset of the options available for `encrypt`:

| Option                    | Description                                                      |
| ------------------------- | ---------------------------------------------------------------- |
| `-p, --path <PATH>`       | Decrypt all subfolders within the source directory. Default: `.` |
| `-i, --include <INCLUDE>` | Decrypt specific directories.                                    |
| `-e, --exclude <EXCLUDE>` | Decrypt all directories except the excluded ones.                |
| `--clean`                 | Delete encrypted folders after decryption.                       |

#### Example

To decrypt all folders in the current directory:

```bash
bage decrypt
```

To decrypt folders in a specific path while excluding some:

```bash
bage decrypt -p dir -e subdir1,subdir2
```

## Option Reference

### Encrypt Options

| Option                    | Description                                                         |
| ------------------------- | ------------------------------------------------------------------- |
| `-p, --path <PATH>`       | Encrypt all subfolders within the source directory. Default: `.`    |
| `-i, --include <INCLUDE>` | Encrypt specific directories (comma-separated).                     |
| `-e, --exclude <EXCLUDE>` | Encrypt all directories except the excluded ones (comma-separated). |
| `-x`                      | Replace filenames with randomized names.                            |
| `-d`                      | Dry run. Preview folders without processing.                        |
| `--clean`                 | Delete source folders after encryption.                             |

### Decrypt Options

| Option                    | Description                                                         |
| ------------------------- | ------------------------------------------------------------------- |
| `-p, --path <PATH>`       | Decrypt all subfolders within the source directory. Default: `.`    |
| `-i, --include <INCLUDE>` | Decrypt specific directories (comma-separated).                     |
| `-e, --exclude <EXCLUDE>` | Decrypt all directories except the excluded ones (comma-separated). |
| `--clean`                 | Delete encrypted folders after decryption.                          |

## Notes

- Always verify your command with the `-d` flag before running encryption on important data.
- The `--clean` option permanently deletes source folders. Use it with caution.
- When combining `-p` and `-i`, the include list is resolved relative to the path specified.
