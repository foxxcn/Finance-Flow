# Finance Flow
**Quickly wrap your web application into an app using Tauri.**

![App Icon](src-tauri/icons/Square310x310Logo.png)

# build

## windows
```cmd
cargo tauri build --bundles nsis –-verbose
```
OR
```cmd
cargo tauri build --bundles msi –-verbose
```
### for Chinese users
```cmd
cargo tauri build --bundles nsis --config '{"bundle":{"windows":{"nsis":{"languages":["SimpChinese"]}}}}' --verbose
```
OR
```cmd
cargo tauri build --bundles msi --config '{"bundle":{"windows":{"wix":{"language":["zh-CN"]}}}}' –-verbose
```

## linux
```bash
cargo tauri build --bundles deb --verbose
```
OR
```bash
cargo tauri build --bundles appimage --verbose
```

## macos
```bash
cargo tauri build --bundles dmg --verbose
```
OR
```bash
cargo tauri build --bundles app --verbose
```