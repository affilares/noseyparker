# Nosey Parker Changelog

This is the changelog for [Nosey Parker](https://github.com/praetorian-inc/noseyparker).
All notable changes to the project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project aspires to use [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## Unreleased

### Additions

- New rules have been added (thank you @gemesa!):

  - Adafruit IO Key ([#114](https://github.com/praetorian-inc/noseyparker/pull/114))
  - Blynk Device Access Token ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Access Token (URL first) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Access Token (URL last) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Client ID (URL first) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Client ID (URL last) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Client Secret (URL first) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Blynk Organization Client Secret (URL last) ([#117](https://github.com/praetorian-inc/noseyparker/pull/117))
  - Docker Hub Personal Access Token ([#108](https://github.com/praetorian-inc/noseyparker/pull/108))
  - Doppler CLI Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Doppler Personal Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Doppler Service Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Doppler Service Account Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Doppler SCIM Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Doppler Audit Token ([#111](https://github.com/praetorian-inc/noseyparker/pull/111))
  - Dropbox Access Token ([#106](https://github.com/praetorian-inc/noseyparker/pull/106))
  - particle.io Access Token (URL first) ([#113](https://github.com/praetorian-inc/noseyparker/pull/113))
  - particle.io Access Token (URL last) ([#113](https://github.com/praetorian-inc/noseyparker/pull/113))
  - ThingsBoard Access Token ([#112](https://github.com/praetorian-inc/noseyparker/pull/112))
  - ThingsBoard Provision Device Key ([#112](https://github.com/praetorian-inc/noseyparker/pull/112))
  - ThingsBoard Provision Device Secret ([#112](https://github.com/praetorian-inc/noseyparker/pull/112))
  - TrueNAS API Key (WebSocket) ([#110](https://github.com/praetorian-inc/noseyparker/pull/110))
  - TrueNAS API Key (REST API) ([#110](https://github.com/praetorian-inc/noseyparker/pull/110))
  - WireGuard Private Key ([#104](https://github.com/praetorian-inc/noseyparker/pull/104))
  - WireGuard Preshared Key ([#104](https://github.com/praetorian-inc/noseyparker/pull/104))

### Fixes
- Fixed several rules that in certain circumstances would fail to match and produce a runtime error message:

  - Google API Key
  - ODBC Connection String
  - Sauce Token


## [v0.16.0](https://github.com/praetorian-inc/noseyparker/releases/v0.16.0) (2023-12-06)

### Additions
- The `scan` command now supports a new `--copy-blobs={all,matching,none}` parameter.
  When specified as `matching`, a copy of each encountered blob that has matches will be saved to the datastore's `blobs` directory.
  When specified as `all`, a copy of _each_ encountered blob will be saved.
  The default value is `none`.
  This mechanism exists to aid in ad-hoc downstream investigation.
  Copied blobs are not used elsewhere in Nosey Parker at this point.

- A new advanced global command-line parameter has been exposed:

  - `--sqlite-cache-size=SIZE` to control the `pragma cache_size` value used in sqlite database connections

- The datastore now contains two additional tables for to represent freeform comments and accept/reject status associated with findings.
  These additional tables are not currently populated in the open-source version of Nosey Parker.
  The `report` command now emits finding status and comment if populated.
  **Note: the datastore format is not settled and is subject to change.**

- A new "ruleset" mechanism has been added.
  A ruleset is a named collection of rules that can be selected as a group.
  The new `--ruleset=NAME` parameter to `scan` can be used to enable alternative rulesets.
  Three built-in rulesets are provided (`default`, `np.assets` and `np.hashes`); the special ruleset name `all` enables all known rules.
  See the built-in rulesets at `crates/noseyparker/data/default/builtin/rulesets` for an example for writing your own.

- The default collection of rules has been pruned down to further emphasize signal-to-noise.
  Only rules that detect secret things are included in the default collection.
  Rules that detect other things, such as cloud assets, application IDs, or public keys, are not included in this set.
  Instead, those are in the `np.assets` ruleset, which is not enabled by default.
  No rules have been removed from Nosey Parker; rather, the defaults have been adjusted to support the most common use case (secrets detection).

- Additional checks have been added to the `rules check` command:

  - Each regex rule must have at least one capture group
  - Each ruleset must have a globally-unique ID
  - A ruleset's included rules must resolve to actual rules
  - A ruleset should not include duplicate rules

- A new `rules list` command is available, which lists available rules and rulesets.
  This command can emit its output in human-oriented format or in JSON format.

- New rules have been added:

  - Dependency-Track API Key (Thank you @tpat13!)
  - Password Hash (sha256crypt)
  - Password Hash (sha512crypt)
  - Password Hash (Cisco IOS PBKDF2 with SHA256)
  - React App Username
  - React App Password

- A new global `--quiet` / `-q` option has been added, which suppresses non-error feedback messages and disables progress bars ([#97](https://github.com/praetorian-inc/noseyparker/issues/97)).

### Fixes
- Command-line parameters that can meaningfully accept negative numbers can now be specified without having to use `--PARAMETER=NEGATIVE_VALUE` syntax; a space can now separate the paraemter and the value.

- Fixed three rules that were missing capture groups:

  - Age Recipient (X25519 public key)
  - Age Identity (X22519 secret key)
  - crates.io API Key

  Due to nuanced details of how scanning is performed, rules without capture groups will never produce reported matches.
  An additional check was added to the `rules check` command and a couple assertions were added that should help prevent this type of error in the future.

- Fixed several rules:

  - Amazon MWS Auth Token: the capture group was smaller than it should have been
  - Microsoft Teams Webhook: changed 3 capture groups to 1; full URL is now included
  - Slack Webhook: full URL is now included

- The LICENSE, README.md, and CHANGELOG.md files are now included in prebuilt binary releases.

- ANSI formatting sequences are now no longer included by default by the `report` command when the output is redirected to a file using the `-o`/`--outfile` parameter ([#55](https://github.com/praetorian-inc/noseyparker/issues/55)).

- The `scan` command should no longer emit warnings like `Failed to decode entry in tree`.
  These warnings were due to a bug in the Git object parsing code in the `gix` dependency, which was fixed upstream.

### Changes
- The `rules check` command invocation now behaves differently.
  It now no longer requires input paths to be specified.
  It will check the built-in rules for problems, and if additional paths are specified, will check those rules as well.
  This change was made so that the `scan`, `rules check`, and `rules list` invocations have consistent interfaces.

- The default path-based ignore rules in Nosey Parker now ignore `packed-refs` files from Git repositories.

- Several rules have been changed:

  - The `Slack` rule (id `np.slack.1`) has been removed, as it was redundant with `Slack Token`.
  - `Slack Token` has been split into `Slack Bot Token`, `Slack Legacy Bot Token`, `Slack User Token`, and `Slack App Token`.
  - `CodeClimate` was enhanced to detect additional cases and was renamed to `CodeClimate Reporter ID`.
  - `md5crypt Hash` (id `np.md5.1`) has been renamed to `Password Hash (md5crypt)` and re-identified as `np.pwhash.1`.
  - `bcrypt Hash` (id `np.bcrypt.1`) has been renamed to `Password Hash (bcrypt)` and re-identified as `np.pwhash.2`.

- Log messages are written to stderr instead of stdout ([#97](https://github.com/praetorian-inc/noseyparker/issues/97)).


## [v0.15.0](https://github.com/praetorian-inc/noseyparker/releases/v0.15.0) (2023-10-12)

### Additions
- A default value (`datastore.np`) is now set for commands that take a datastore parameter ([#74](https://github.com/praetorian-inc/noseyparker/issues/74)).
  This makes simpler `noseyparker` command-line invocations possible.

- A new `shell-completions` command has been added, which generates shell-specific completion scripts for zsh, bash, fish, powershell, and elvish ([#76](https://github.com/praetorian-inc/noseyparker/pull/76)).
  These generated completion scripts make discovery of Nosey Parker's command-line API simpler.
  Thank you @Coruscant11!

- The `report` command supports a new `--max-matches=N` parameter to control the maximum number of matches that will be output for any single finding ([#75](https://github.com/praetorian-inc/noseyparker/issues/75)).
  A negative number means "no limit".

- The `scan` command now supports a new `--git-history={full,none}` parameter to control whether encountered Git history will be scanned.
  This defaults to `full`, but specifying a value of `none` will cause Git history to be ignored.

- New rules have been added:

  - Mapbox Temporary Access Token
  - Salesforce Access Token

- A new `disable_tracing` Cargo feature has been added, which disables `trace`-level logging and tracing messages.
  This feature is also aliased by a new `release` feature, which is enabled in prebuilt releases.

- The `NP_LOG` environment variable is inspected at runtime to allow find-grain control over Nosey Parker's diagnostic output.
  The syntax of this variable are defined by the [`tracing-subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) Rust crate.

### Changes
- All the output formats for the `report` command now respect the new `--max-matches=N` parameter.
  Previously, the output formats other than `human` would run without limit (i.e., as though `--max-matches=-1` had been specified).

- The release process is now codified in a shell script: `scripts/create-release.zsh`.
  This emits a release tree at `release` in the top-level of the repository, which includes the prebuilt binary as well as shell completions ([#80](https://github.com/praetorian-inc/noseyparker/issues/80)).

- The `report` command has improved performance when using JSON output format.
  Previously, the entire JSON output document needed to be accumulated in memory and then written in one step at the end.
  Now, the JSON output document is written in a streaming fashion, one finding at a time.

- `mimalloc` is now used as the global allocator ([#81](https://github.com/praetorian-inc/noseyparker/issues/81)).
  This reduces peak resident memory when scanning large inputs with a high degree of parallelism.

### Fixes
- Fixed a bug in the `report` command when `--format=sarif` is used which caused some metadata to be unintentionally omitted from the output.


## [v0.14.0](https://github.com/praetorian-inc/noseyparker/releases/v0.14.0) (2023-08-17)

A [prebuilt multiplatform Docker image](https://github.com/praetorian-inc/noseyparker/pkgs/container/noseyparker/119700654?tag=v0.14.0) for this release is available for x86_64 and ARM64 architectures:
```
docker pull ghcr.io/praetorian-inc/noseyparker:v0.14.0
```

### Additions
- Running `noseyparker --version` now emits many compile-time details about the build, which can be useful for troubleshooting ([#48](https://github.com/praetorian-inc/noseyparker/issues/48)).

- The `github` and `scan` commands now support accessing GitHub Enterprise Server instances using the new `--github-api-url URL` parameter ([#53](https://github.com/praetorian-inc/noseyparker/pull/53)—thank you @AdnaneKhan!).

- New rules have been added:

  - Amazon Resource Name
  - AWS S3 Bucket (subdomain style)
  - AWS S3 Bucket (path style)
  - Google Cloud Storage Bucket (subdomain style)
  - Google Cloud Storage Bucket (path style)
  - HuggingFace User Access Token ([#54](https://github.com/praetorian-inc/noseyparker/pull/54)—thank you @AdnaneKhan!)

- Rules are now required to have a globally-unique identifier ([#62](https://github.com/praetorian-inc/noseyparker/pull/62))

- Two new advanced global command-line parameters have been exposed:

  - `--rlimit-nofile LIMIT` to control the maximum number of open file descriptors
  - `--enable-backtraces BOOL` to control whether backtraces are printed upon panic

- The snippet length for matches found by the `scan` command can now be controlled with the new `--snippet-length BYTES` parameter.

- The Git repository cloning behavior in the `scan` command can now be controlled with the new `--git-clone-mode {mirror,bare}` parameter.

- The `scan` command now collects additional metadata about blobs.
  This metadata includes size in bytes and guessed mime type based on filename extension.
  Optionally, if the non-default `libmagic` Cargo feature is enabled, the mime type and charset are guessed by passing the content of the blob through `libmagic` (the guts of the `file` command-line program).

  By default, all this additional metadata is recorded into the datastore for each blob in which matches are found.
  This can be more precisely controlled using the new `--blob-metadata={all,matching,none}` parameter.

  This newly-collected metadata is included in output of the `report` command.

- The `scan` command now collects additional metadata about blobs found within Git repositories.
  Specifically, for each blob found in Git repository history, the set of commits where it was introduced and the accompanying pathname for the blob is collected ([#16](https://github.com/praetorian-inc/noseyparker/issues/16)).
  This is enabled by default, but can be controlled using the new `--git-blob-provenance={first-seen,minimal}` parameter.

  This newly-collected metadata is included in output of the `report` command.

### Changes
- The datastore schema has been changed in an incompatible way such that migrating existing datastores to the new version is not possible.
  This was necessary to support the significantly increased metadata that is now collected when scanning.
  Datastores from earlier releases of Nosey Parker cannot be used with this release; instead, the inputs will have to be rescanned with a new datastore.

- The JSON and JSONL output formats for the `report` command have changed slightly.
  In particular, the `.matches[].provenance` field is now an array of objects instead of a single object, making it possible to handle situations where a blob is discovered multiple ways.
  The `provenenance` objects have some renamed fields, and contain significantly more metadata than before.


- Existing rules were modified to reduce both false positives and false negatives:

  - Generic Password (double quoted)
  - Generic Password (single quoted)

- The default size of match snippets has been increased from 128 bytes before and after to 256.
  This typically gives 4-7 lines of context before and after each match.

- When a Git repository is cloned, the default behavior is to match `git clone --bare` instead of `git clone --mirror`.
  This new default behavior results in cloning potentially less content, but avoids cloning content from forks from repositories hosted on GitHub.

- The command-line help has been refined for clarity.

- Scanning performance has been improved on particular workloads by as much as 2x by recording matches to the datastore in larger batches.
  This is particularly relevant to heavy multithreaded scanning workloads where the inputs have many matches.

### Fixes
- Python is no longer required as a build-time dependency for `vectorscan-sys`.

- A typo was fixed in the Okta API Key rule that caused it to truncate the secret.

- The `scan` command now correctly reports the number of newly-seen matches when reusing an existing datastore.


## [v0.13.0](https://github.com/praetorian-inc/noseyparker/releases/v0.13.0) (2023-04-24)

A [prebuilt multiplatform Docker image](https://github.com/praetorian-inc/noseyparker/pkgs/container/noseyparker/88043263?tag=v0.13.0) for this release is available for x86_64 and ARM64 architectures:
```
docker pull ghcr.io/praetorian-inc/noseyparker:v0.13.0
```

### Changes
- Nosey Parker now statically links against a bundled version of [Vectorscan](https://github.com/Vectorcamp/vectorscan) for regular expression matching instead of [Hyperscan](https://github.com/intel/hyperscan) ([#5](https://github.com/praetorian-inc/noseyparker/issues/5)).
  This makes building from source simpler, particularly for ARM-based platforms.
  This also simplifies distribution, as a precompiled `noseyparker` binary now has no runtime library dependencies on non-default libraries.

- Several existing rules were modified to reduce false positives and false negatives:

  - Generic API Key
  - Telegram Bot Token

### Additions
- New rules have been added:

  - Generic Username and Password (quoted)
  - Generic Username and Password (unquoted)
  - Generic Password (double quoted)
  - Generic Password (single quoted)
  - Grafana API Token
  - Grafana Cloud API Token
  - Grafana Service Account Token
  - Postman API Key

- References have been added for several rules:

  - Twilio API Key
  - Dynatrace Token

### Fixes
- The Docker image now has the `git` binary installed. Previously this was missing, causing the `scan` command to fail when the `--git-url`, `--github-user`, or `--github-organization` input specifiers were used ([#38](https://github.com/praetorian-inc/noseyparker/issues/38)).


## [v0.12.0](https://github.com/praetorian-inc/noseyparker/releases/v0.12.0) (2023-03-02)

A [prebuilt Docker image](https://github.com/praetorian-inc/noseyparker/pkgs/container/noseyparker/74541424?tag=v0.12.0) for this release is available for x86_64 architectures:
```
docker pull ghcr.io/praetorian-inc/noseyparker:v0.12.0
```

### Additions
- The `scan` command can now be given Git https URLs, GitHub usernames, and GitHub organization names as inputs, and will enumerate, clone, and scan as appropriate ([#14](https://github.com/praetorian-inc/noseyparker/issues/14)).

- Nosey Parker now has rudimentary support for enumerating repositories from GitHub users and organizations ([#15](https://github.com/praetorian-inc/noseyparker/issues/15)).
  The new `github repos list` command uses the GitHub REST API to enumerate repositories belonging to one or more users or organizations.
  An optional GitHub Personal Access Token can be provided via the `NP_GITHUB_TOKEN` environment variable.

- Nosey Parker now has an optional `rule_profiling` crate feature that causes performance-related statistics to be collected and reported when scanning.
  This feature imposes some performance cost and is only useful to rule authors, and so is disabled by default.

- Many new rules have been added:

  - Adobe OAuth Client Secret
  - Age Identity (X22519 secret key)
  - Age Recipient (X25519 public key)
  - crates.io API Key
  - DigitalOcean Application Access Token
  - DigitalOcean Personal Access Token
  - DigitalOcean Refresh Token
  - Figma Personal Access Token
  - GitLab Personal Access Token
  - GitLab Pipeline Trigger Token
  - GitLab Runner Registration Token
  - Google OAuth Client Secret (prefixed)
  - New Relic API Service Key
  - New Relic Admin API Key
  - New Relic Insights Insert Key
  - New Relic Insights Query Key
  - New Relic License Key
  - New Relic License Key (non-suffixed)
  - New Relic Pixie API Key
  - New Relic Pixie Deploy Key
  - New Relic REST API Key
  - NPM Access Token (fine-grained)
  - OpenAI API Key
  - Segment Public API Token
  - Shopify Access Token (Custom App)
  - Shopify Access Token (Legacy Private App)
  - Shopify Access Token (Public App)
  - Shopify App Secret
  - Shopify Domain
  - RubyGems API Key
  - Telegram Bot Token

  These rules match token formats that are well-specified fixed-length strings with notable prefixes or suffixes, and so should produce very few false positives.

- Several existing rules were modified to improve signal-to-noise:

  - Azure Connection String
  - Credentials in ODBC Connection String
  - PyPI Upload Token

- The `report` command now offers rudimentary SARIF support ([#4](https://github.com/praetorian-inc/noseyparker/issues/4)).
  Thank you @Coruscant11!

### Changes
- Several default rules have been revised to improve performance of the matching engine and to produce fewer false positives.
  In particular, several rules previously had avoided using a trailing `\b` anchor after secret content which could include a literal `-` character, due to a matching discrepancy between Hyperscan and Rust's `regex` library.
  These have been revised to use a more complicated but functional anchoring pattern.

- The `JSON Web Token (base64url-encoded)` rule has been changed to only produce a single match group instead of three.

- The `Google Client Secret` rule has been improved to detect additional occurrences and has been renamed to `Google OAuth Client Secret`.

- Blobs are now deduplicated at enumeration time when first enumerating a Git repository, rather than only at scan time. This results in more accurate progress bars.

- When scanning, Git repositories are now opened twice: once at input enumeration time, and once at scanning time.
  This drastically reduces the amount of memory required to scan a large number of Git repositories.

### Fixes
- When scanning, the datastore is now explicitly excluded from filesystem enumeration.
  This ensures that files used internally for Nosey Parker's operation are not inadvertently scanned ([#32](https://github.com/praetorian-inc/noseyparker/issues/32)).


## [v0.11.0](https://github.com/praetorian-inc/noseyparker/releases/v0.11.0) (2022-12-30)

This is the first released version of Nosey Parker.
Its `scan`, `summarize`, and `report` commands are functional.
It is able to scan files, directories, and the complete history of Git repositories at several hundred megabytes per second per core.
It comes with 58 rules.

A [prebuilt Docker image](https://github.com/praetorian-inc/noseyparker/pkgs/container/noseyparker/61045424?tag=v0.11.0) for this release is available for x86_64 architectures:
```shell
docker pull ghcr.io/praetorian-inc/noseyparker:v0.11.0
```
