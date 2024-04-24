# Tauri Plugin openurl

Open url in browser on all platforms<sup>1</sup> (excluding IOS, still in testing). Basically `<a target="_blank"></a>` replacement for all platforms<sup>1</sup>.


## Usage

This plugin is still part of Tauri V2 Beta. So, this project might be replaced by a fix in Tauri core itself before project is stable.

### Setup
npm

```bash
npm i tauri-plugin-openurl-api
```

&& cargo (possibly inside ./src-tauri/)
```bash
cargo add tauri-plugin-openurl
```

### Usage

```javascript
import { open_url } from "tauri-plugin-openurl-api";

<button onClick={() => {
    open_url("https://crates.io/crates/tauri-plugin-openurl");
}}>Click to open in default browser</button>
```

### Permissions
Please make sure you add this permission to all platforms you intend to use in
```json
{
  "permissions": [
    "openurl:allow-open-url"
  ]
}
```







References:

<sup>1</sup> : excluding IOS as it is still in testing.