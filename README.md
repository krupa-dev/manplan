# Manplan - SDKman Manifest Helper

This tool reads a YAML manifest defining a list of SDKman candidate
versions and installs the latest versions that match this manifest.

Each candidate can support multiple versions and each version is
specified as a regular expression match and zero or more regular
expression exclusions (in order to, for example, filter out release
candidates and alpha versions). One version for each candidate can
also be specified as the global default for that candidate.

Candidate versions that do not match one of the rules in the
manifest will be uninstalled unless the -n|--no-uninstall flag is
specified. Candidates that are not listed in the manifest will be
ignored.

## Usage

`manplan -f|--file <manifest file> [-d|--dry-run]`

## Manifest format

The format is simple. The root object is called 'candidates' which
contains a map of candidate name to definitions. Each definition
has a single attribute 'versions' which is a list of version. Each
version has a single attribute 'pattern' that is a regular expression
to match for expected versions (e.g. `^1\.8\..*$`) and optional 'default'
(a boolean indicating if this is the default version) and 'exclude'
(a list of exlucded regaulr expressions such as '.*-rc.*') fields.

### Example manifest

```yaml
candidates:
  java:
    versions:
      - pattern: "21.*-zulu"
        default: true
      - pattern: "21.*-graalce"
      - pattern: "17.*-graalce"
      - pattern: "17.*-zulu"
      - pattern: "8.*-zulu"
  kotlin:
    versions:
      - pattern: ".*"
        default: true
  groovy:
    versions:
      - pattern: ".*"
        exclude:
          - ".*alpha.*"
          - ".*-rc.*"
        default: true
  gradle:
    versions:
      - pattern: ".*"
        exclude:
          - ".*-rc.*"
        default: true
  maven:
    versions:
      - pattern: "3.*"
        exclude:
          - ".*alpha.*"
        default: true
  quarkus:
    versions:
      - pattern: ".*"
        default: true
  micronaut:
    versions:
      - pattern: ".*"
        exlcude:
          - "^.*M.*$"
          - "^.*RC.*$"
```
