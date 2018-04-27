# ZLNK
[![](https://img.shields.io/github/license/zitrone44/zlnk.svg?style=flat-square)](https://github.com/Zitrone44/zlnk/blob/master/LICENSE)

## Installation
### Precompiled
Precompiled binaries are available at the [releases](https://github.com/Zitrone44/zlnk/releases) page.

### From Source
**Requirements:**
* git
* rust (nightly)
* cargo

```
git clone https://github.com/Zitrone44/zlnk.git
cd zlnk
cargo build --release
```

## Configuration
### Enviroment Variables
|Name|Default|Possible Values|Description|
|----|-------|---------------|-----------|
|REDIS_URL|`redis://localhost`|Any redis URI (`redis://host:port/db`)|The URL of the redis instance zlnk should use|
|URL_REGEX|`https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)`|Any rust RegEX|Only URLs that match this regex can be shortend|
|SHORT_LENGTH|`5`|Any int (0 < i < 2^64)|Length of an short url|
|SHORT_ALPHABET|`hex`|`hex`, `decimal`, `alpha`, `alpha-numeric`|Alphabet used in short urls|
|BAD_REQUEST_MESSAGE|`Invalid URL`|Any String|Error message if submitted url does not matches the regex|
|MMDB_PATH|`./GeoLite2-Country.mmdb`|Any Path to an mmdb file|Path to the mmdb file used for country detection|
|TRUST_PROXY|`false`|Set => `true`, Not Set => `false`|If set the `X-Forwarded-For` header value is used as request ip|
|DISABLE_STATS|`false`|Set => `true`, Not Set => `false`|If set no stats are collected|
