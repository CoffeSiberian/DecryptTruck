# DecryptTruck

Decrypt your saves in the ATS and ETS 2 games with this quick little utility rewritten in Rust.

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

## Credits

This repository is a rewrite of other projects in order to continue to support the ETS 2 and ATS save decryption. All credits for logic and discoveries to:

### SII_Decrypt

Original project where there is a lot of documentation https://github.com/TheLazyTomcat/SII_Decrypt

### SII DecryptSharp

Project where most of the logic is based on https://gitlab.com/jammerxd/sii-decryptsharp
