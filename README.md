# Introducing the Pendora CLI
The job of this tool is to allow for two primary tasks:
- Creating and structuring Pendora projects
- Providing access to the transpiler for generating build artifacts

## Installation
We do not currently provide any public builds of the CLI itself.
You will need to clone the repo and use Cargo to build it.
This also allows you to change the version of the parsing framework, [Pendora Base](https://github.com/PendoraProject/pendora-base) in Cargo.toml.
However, it is recommended to always use the latest version, unless said otherwise.
```bash
git clone https://github.com/PendoraProject/pendora-cli.git
cd pendora-cli
cargo build --release

```

## Language support
Currently we only support transpilation for Rust but are working on Python and other language support.
See the Implementation Documentation section of the Pendora Docs for more information.

## Quick Guide
A quick reference for commands
### Create a New Project
`<name>` field is optional
```bash
pendora-cli new <name>

```
### Create a New Object
`<name>` field is optional
```bash
pendora-cli create object <name>

```
### Create a New Method
`<name>` field is optional
```bash
pendora-cli create method <name>

```
### Build all artifacts
```bash
pendora-cli build

```
### Build artifacts for specific language
```bash
pendora-cli build -L="<lang>"
# or
pendora-cli build --lang="<lang>"

```
