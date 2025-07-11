## Description
Aesencrypt

## Installation
Copy file from releases into /usr/bin

You might need to run chmod uga+x /usr/bin/aesencrypt

## Parameters
| Parameter      | Description |
| ----------- | ----------- |
| --input-file | Input file to encrypt or decrypt. |
| --output-file | Output file after encryption or decryption. |
| --password | Password to use in encryption/decryption. |
| --encrypt | Encrypt. |
| --decrypt | Decrypt. |

## Encryption
```
aesencrypt --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --password <PASSWORD> --encrypt
```

## Decryption
```
aesencrypt --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --password <PASSWORD> --decrypt
```