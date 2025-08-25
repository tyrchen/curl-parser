# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.5.0](https://github.com/tyrchen/curl-parser/compare/v0.4.4..v0.5.0) - 2025-01-07

### Features

- do not parse url to uri for certain use cases (e.g. jinja2 templating) - ([2f98391](https://github.com/tyrchen/curl-parser/commit/2f983919fa0fda19b964aee334daa8b9e5353d15)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([9019e3d](https://github.com/tyrchen/curl-parser/commit/9019e3db3a388b3a1a2fbf07f822c5ceadec7ade)) - Tyr Chen

---
## [0.4.4](https://github.com/tyrchen/curl-parser/compare/v0.4.3..v0.4.4) - 2025-01-06

### Bug Fixes

- parse body in different manner - ([73dda54](https://github.com/tyrchen/curl-parser/commit/73dda5489da8bacfb366bdcc61ae4de96b537031)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([d38fc38](https://github.com/tyrchen/curl-parser/commit/d38fc3811116f3215f482323cdb63376260020a2)) - Tyr Chen

---
## [0.4.3](https://github.com/tyrchen/curl-parser/compare/v0.4.2..v0.4.3) - 2025-01-06

### Bug Fixes

- if body contains a url which has '-', the grammar will break - ([ef082ab](https://github.com/tyrchen/curl-parser/commit/ef082abff004f4aa286d7ff73ef2e67ffbda4010)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([bea3ee3](https://github.com/tyrchen/curl-parser/commit/bea3ee30952f784d5122f5eb53bc9c7b1da007f8)) - Tyr Chen

---
## [0.4.2](https://github.com/tyrchen/curl-parser/compare/v0.4.1..v0.4.2) - 2025-01-05

### Features

- allow shell comments before the curl command - ([b59e285](https://github.com/tyrchen/curl-parser/commit/b59e28525fc5d76cd53f1e8d32d6809a6faab3b1)) - Tyr Chen

### Miscellaneous Chores

- add updated precommit and update gh action - ([865271d](https://github.com/tyrchen/curl-parser/commit/865271d0a95cc21f80fb07e7e609f195bee74dbc)) - Tyr Chen

---
## [0.4.1](https://github.com/tyrchen/curl-parser/compare/v0.4.0..v0.4.1) - 2025-01-04

### Miscellaneous Chores

- provide FromStr and make load() easiler to use. - ([1ed3ae5](https://github.com/tyrchen/curl-parser/commit/1ed3ae52fd9349bc15bc613fdeea564d56242ebd)) - Tyr Chen

---
## [0.4.0](https://github.com/tyrchen/curl-parser/compare/v0.3.1..v0.4.0) - 2025-01-04

### Miscellaneous Chores

- add doc and use TryFrom to convert a ParsedRequest to RequestBuilder - ([9f76e25](https://github.com/tyrchen/curl-parser/commit/9f76e256ecb56c5d477b64ed7763d2e578c2bc00)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([5caca5e](https://github.com/tyrchen/curl-parser/commit/5caca5e7c6fbf7f528b4ed3d480573ca26b20b0d)) - Tyr Chen
- Support for Insecure Option and Dependency Updates (#8)

This pull request introduces the following key changes:

### Features:
- **Insecure Option Support**: Implements the `-k` or `--insecure`
option in the curl command parser. This feature allows users to bypass
SSL certificate verification, enhancing flexibility for testing and
development environments where self-signed certificates are common. It's
crucial to note that this option should be used cautiously, especially
in production environments, due to the security implications of
disabling SSL verification.

### Improvements:
- **Dependency Updates**: Upgraded various dependencies to their latest
versions. This update is part of our ongoing effort to maintain the
project's security and performance by staying up-to-date with the latest
fixes and features provided by our dependencies. The updates include
critical security patches and performance enhancements that contribute
to the overall robustness of our application.


### Testing:
- Added comprehensive tests for the new insecure option to ensure its
correct behavior. These tests validate that the SSL certificate
verification is appropriately bypassed when the `-k` or `--insecure`
option is used.
- Updated existing tests to accommodate the changes introduced by the
dependency updates. Ensured that all tests pass with the updated
libraries, confirming compatibility and stability.

---------

Signed-off-by: deadash <dead.ash@qq.com> - ([d6a8df0](https://github.com/tyrchen/curl-parser/commit/d6a8df0cd0fc2b8dddb1eb53775b208a9c216ba9)) - deadash

### Refactoring

- **(README)** update curl parsing example in README (#11) - ([8d50a6f](https://github.com/tyrchen/curl-parser/commit/8d50a6f15623ad5ebd5e57252ed2e281c0b70726)) - edc-lib

---
## [0.3.1](https://github.com/tyrchen/curl-parser/compare/v0.3.0..v0.3.1) - 2023-12-18

### Bug Fixes

- parse quoted url, set default http:// scheme for url (#3) - ([1149953](https://github.com/tyrchen/curl-parser/commit/1149953ecd024664828f44c84908e47e278762dd)) - Evgeniy Tatarkin
- code format - ([fc3ae2b](https://github.com/tyrchen/curl-parser/commit/fc3ae2bead0c7c1c109e113676e496d9766ebf29)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([ada544e](https://github.com/tyrchen/curl-parser/commit/ada544ed56106a6ee229b5d362429a55b7266be6)) - Tyr Chen
- add location option (#1)

Hi
I just want to add the location option to the code
It probably does not do much besides replacing the url, but the thing is
a curl generated by Postman always have the ```--location``` option,
which I think is something to think about.
Not that versed in Rust and Pest yet, so please improve my code if able.
Also, I just add the ```---request``` option along with the ```-X```
And should the ```curl``` request have data but it is a ```GET```,[ it
should be automatically converted to a
```POST```](https://reqbin.com/req/c-g5d14cew/curl-post-example).
Thank you.

---------

Co-authored-by: Cường Nguyễn (Software Engineer) <cuong.nguyen4@be.com.vn>
Co-authored-by: Tyr Chen <tyr.chen@gmail.com> - ([52ed623](https://github.com/tyrchen/curl-parser/commit/52ed6235fa91643c3dfa3602670bbe53ec053592)) - ffleader1
- bump version to 0.3.1 - ([cd6ee9d](https://github.com/tyrchen/curl-parser/commit/cd6ee9dbec7e0802d284dadbd4724dae59e9d247)) - Tyr Chen
- Update CHANGELOG.md - ([19f850d](https://github.com/tyrchen/curl-parser/commit/19f850d99034e3670527f128b7a998b1db2297ce)) - Tyr Chen

---
## [0.3.0](https://github.com/tyrchen/curl-parser/compare/v0.2.1..v0.3.0) - 2023-12-18

### Miscellaneous Chores

- update deps and bump version to 0.3 - ([6861c72](https://github.com/tyrchen/curl-parser/commit/6861c72d721e409398fd366936e3062a66dc8878)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([a8cc54d](https://github.com/tyrchen/curl-parser/commit/a8cc54d38e9f3334fefbbf33ab9e90cd82fdc2ab)) - Tyr Chen

---
## [0.2.1](https://github.com/tyrchen/curl-parser/compare/v0.2.0..v0.2.1) - 2023-02-09

### Bug Fixes

- disable json feature for minijinja - ([9632f1c](https://github.com/tyrchen/curl-parser/commit/9632f1c93e149bf58ee2a227f004e602019d2588)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([0c7e942](https://github.com/tyrchen/curl-parser/commit/0c7e942c289b1ab539794b2be9f5713d440ac783)) - Tyr Chen

---
## [0.2.0](https://github.com/tyrchen/curl-parser/compare/v0.1.1..v0.2.0) - 2023-02-09

### Features

- support template rendering so that variables in curl command could be pre-processed - ([26d7054](https://github.com/tyrchen/curl-parser/commit/26d7054f8c5fbf56d3c57ad8008503e775efe6f3)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([258b0bf](https://github.com/tyrchen/curl-parser/commit/258b0bf872ae99d380a526cec862f9b46e99c2eb)) - Tyr Chen

---
## [0.1.1](https://github.com/tyrchen/curl-parser/compare/v0.1.0..v0.1.1) - 2023-02-09

### Miscellaneous Chores

- remove user-agent - ([163d9e3](https://github.com/tyrchen/curl-parser/commit/163d9e301ffb9dda0325bcf4fac1053873f5cc95)) - Tyr Chen

---
## [0.1.0] - 2023-02-09

### Features

- MVP to convert curl command to a ParsedRequest - ([d62e21b](https://github.com/tyrchen/curl-parser/commit/d62e21bddbccd74efb0edae69146183105150348)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([b55a4e5](https://github.com/tyrchen/curl-parser/commit/b55a4e519b0124bd4f65b4784fca9183c2fa1fcb)) - Tyr Chen

<!-- generated by git-cliff -->
