# Problem statement
Currently I am running into problems with my file system. When my files have multiple attributes (e.g. something is both a rust program and a control algorithm). I am unable to hierarchically organise my files without excessive symlinks which I dislike. I want to be able to tag files, with similiar functionality to TagStudio. 
- Can add attributes to attributes
- Able to search files by tag
    - name
    - alias'
- Should work on all UNIX based OS's

There is also some additional functionality I want to implement
- automatic timestamp tag (search by year, year+month, year+month+day)
    - Would be created on first creation of that tag
    - Can be edited manually and with command
- CLI
- Hashing of files to check for renaming (only works if contents are unchanged)
- Tag metadata is bundled horizontally as `CRTag.toml` sidecar files
- Changes made are pushed out to the filesystem (error raised if conflict found)

CRTag must be run inside a directory that contains a `CRTagDefinitions.toml` which will hold all the tags and aliases. The program will attempt to search upwards for these definitions but will error if not found.

If a file cannot be found it searches for matching names and/or hashes. If either or those are found in the search path it suggests the moving of relevant tags. Otherwise it continues on and prints that a missing file was encountered.

All tags can only be ASCII symbols.

# Commands to implement
## init
```zsh
crtag init
crtag init <path>
```
Creates the CRTagDefinitions at the specified path

## add
```zsh
crtag add filename.txt <tags>
crtag add directoryname <tags>
```
Adds tags to relevant targets and creates the `CRTag.toml` if necessary.
Errors on tags not being known

## find
```zsh
crtag find <search_terms> --path pathname
crtag find <search_terms> -p pathname
```
Searches for the search terms in the following characteristics, inside the specified path
- filename
- text inside plaintext documents
- tags, subtags and tag aliases (case insensitive)
After finding matches they are all printed out

## date
```zsh
crtag date filename.txt "now"
crtag date filename.txt "18-May-2025"
```
This will add either the current date or the specified date to the alternate_times. If the input is invalid it will error

## subtag
```zsh
crtag subtag <supertag> <subtag>
crtag subtag coding rust
crtag subtag languages rust
```
Tags the relevant tag with the supertag so all searches of the supertag return the tag.
A single tag can have multiple supertags and vice versa. This will create the tag and supertag if they do not exist.
This is case-sensitive, and errors on tags not being known.

## check
```zsh
crtag check
crtag check file_path
```
Searches for misplaced files or changes and attempts to find where they have gone. Returning the results

## new
```zsh
crtag new tag1 tag2
```
Creates new tags

# File structure
## CRTagDefinitions
```toml
[rust]
version = "1.0.0" # Describes the semantic versioning of the program version that created this tag
aliases = ["rust", "rustacean", "ferris"] # Shorthands and alternate names

[coding]
version = "1.0.0"
aliases = ["coding", "code"]
subtags = ["rust"]

[languages]
version = "1.0.0"
aliases = ["languages"]
subtags = ["rust"]
```

## CRTag
```toml
[myfile1]
hash = "644bcc7e564373040999aac89e7622f3ca71fba1d972fd94a31c3bfbf24e3938"
tags = ["rust", "coding"]
version = "1.0.0" # Describes the semantic versioning of the program that tagged this file

[mydirectory]
# There is no hash for directories
tags = ["art", "painting", "fireworks"]
version = "1.0.0"

[mydirectory.timing]
creation_time = "18-May-2025"
alternate_times = ["25-May-2025", "01-Jan-2100"] # Added by the user
```

# Current Limitations
- Currently only `dd-mmm-yyyy` date formats are supported