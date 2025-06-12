# Problem statement
Currently I am running into problems with my file system. When my files have multiple attributes (e.g. something is both a rust program and a control algorithm). I am unable to hierarchically organise my files without excessive symlinks which I dislike. I want to be able to tag directories and then search these tags.

# Features
- Can add attributes to attributes
- Able to search directories by tag
    - name
    - alias'
- Should work on all UNIX based OS's
- CLI
- Tag metadata is bundled inside the tagged directory as `CRTag.toml` sidecar files inside `.crtag` directories
- Changes made are pushed out to the filesystem

CRTag must be run inside a directory that contains a `CRTagDefinitions.toml` which will hold all the tags and aliases. The program will attempt to search upwards for these definitions but will error if not found.

All tags can only be ASCII symbols. This is not enforced but the program is not guaranteed to work with other character sets.

# Commands
## init
```zsh
crtag init # Will run in the current directory
crtag init <path>
```
Creates the CRTagDefinitions at the specified path

## add
```zsh
crtag add directoryname <tags>
```
Adds tags to relevant directory and creates the `CRTag.toml` if necessary.
Errors if it encounters a unknown tag, tags that are allowable are still added.

## find
```zsh
crtag find <search_terms>
```
Searches for the tag or its subtags in the `CRTag.toml` files, down from `CRTagDefinitions.toml`
After finding matches they are all printed out.

## subtag
```zsh
crtag subtag <supertag> <subtags>
crtag subtag coding rust
crtag subtag languages rust
```
Tags the relevant tag with the supertag so all searches of the supertag return the tag.
A single tag can have multiple supertags and vice versa. This will create the tag and supertag if they do not exist.
This is case-sensitive, and errors on tags not being known.

## new
```zsh
crtag new tag1 tag2
```
Creates new tags

## version
```zsh
crtag version
```
Prints out the version of crtag that is running

# File structure
## CRTagDefinitions
```toml
[coding]
aliases = ["coding", "code"] # Shorthands and alternate names
subtags = ["rust"]
version = "1.0.0" # Describes the semantic versioning of the program version that created this tag

[languages]
aliases = ["languages"]
subtags = ["rust"]
version = "1.0.0"

[rust]
aliases = ["rust", "rustacean", "ferris"]
version = "1.0.0"

```

## CRTag
```toml
tags = ["rust", "coding"]
version = "1.0.0" # Describes the semantic versioning of the program that tagged this file
```

# Stuff to debug
- Currently search doesn't work
- aliases are not implemented
- rework to only target directories
- rework so that if a invalid tag is encountered all other tags are still added

# Future improvement
If continue with this, I might consider the following
- creating types and associated methods for tag and definitions files instead of a function based method.