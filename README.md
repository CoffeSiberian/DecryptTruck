# DecryptTruck

This script facilitates the decryption of the ATS and ETS 2 save game by making use of the SII Decrypt .dll library for those cases in which the saves cannot be decrypted by the normal console application.

## How to use

The script has 2 parameters

-   FilePath
-   FilePathSave (Optional)

Paths can be absolute or relative. Drag and drop on the executable is also supported.

## Download

To download you can obtain the archive at https://github.com/CoffeSiberian/DecryptTruck/releases/latest where the executable and the SII Decrypt library are located.

## Build App

In order to compile the application you will first need to install Rust (https://www.rust-lang.org/tools/install) and most probably Visual Studio C++ Build tools.

Then you will need to use this command to start the build.

```
cargo build --release
```

Once the process is finished. The binary will remain in `./target/release/decrypt_truck.exe` remember to also download your SII_Decrypt.dll for the tool to work correctly.

### Project SII_Decrypt

This repository makes use of SII Decrypt. A library that allows access to ATS and ETS 2 save games.
https://github.com/TheLazyTomcat/SII_Decrypt
