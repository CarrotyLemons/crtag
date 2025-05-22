# Problem statement
Currently I am running into problems with my file system. When my files have multiple attributes (e.g. something is both a rust program and a control algorithm). I am unable to hierarchically organise my files without losing information. I want to be able to tag files, with similiar functionality to TagStudio. 
- Can add attributes to attributes
- Able to search files by tag
    - name
    - alias
    - shorthand
- Tag metadata is bundled horizontally along with files
- Absolutely must be system interoperable (CLI should work in any bash shell)

Some functionality I want to have that TagStudio does not include
- automatic timestamp tag (serach by year, year+month, year+month+day)
    - Would be created on first creation of that tag
    - Can be edited manually and with command
- CLI
- Hashing of files to check for renaming (only works if contents are unchanged)
- Information to be stored in sidecar files (`CarroTag.toml`?), nothing stored in CarroTag software package
- Changes made are pushed out to the filesystem (error raised if conflict found)
- Human readable information storage format (intersystem operability when no CarroTag available)
- Created with rust
- This allows a very fast CarroTag interface while preserving human readability

This would allow me to structure folders up until I hit that attribute issue, at which point all further files and directories are organised flat and held at the same level in the same directory. At this point a metadata file would take over and be read by CarroTag.

CarroTag must be run inside a directory that contains a `CarroTagDefinitions.toml` which will hold all the tags and aliases. The program will attempt to serach upwards but will error if not found.

If a file cannot be found (moved) it searches for matching names and/or hashes. If either or those are found it in search path it suggests the moving of relevant tags. Otherwise it continues on and prints that a missing file was encountered.

All tags can only be letters underscores and numbers
`(?:[a-z]|[A-Z]|_|[0-9])+`

# Functionality to implement
## General functionality
Terminal text output colouring

## Commands to implement
```zsh
carrotag add filename.txt tag1 tag2
carrotag add directoryname tag1 tag2 tag3
```
Adds tags to relevant directory and creates 


```zsh
carrotag find search_term --path pathname
```

Searches for all files named with search_term in path recursively
- looks for filename
- looks for text inside relevant documents (`.txt` and `.md`)
- looks for tags and tag aliases (not case sensitive)
- if pathname is specified it searches in that directory

Will print out a list of all directories/files that are a match

```zsh
carrotag date filename.txt "now"
carrotag date filename.txt "18-May-2025"
```
This will add either the current date or the specified date to the alternate_times. If the input is invalid it will error

```zsh
carrotag subtag tag supertag
carrotag subtag rust coding
```
Tags the relevant tag with the supertag so all searches of the supertag return the tag.
A single tag can have multiple supertags and vice versa. This will create the tag and supertag if they do not exist.

```zsh
carrotag check
carrotag check file_path
```
Searches for misplaced files or changes and returns them

```zsh
carrotag new tag1 tag2
```
Creates new tags
# Implemented commands

# File structure
## CarroTagDefinitions
```toml
[rust]
version = "1.0.0"
aliases = ["rust", "rustacean", "ferris"]
supertags = ["coding", "languages"]

[coding]
version = "1.0.0"
aliases = ["coding", "code"]

[languages]
version = "1.0.0"
aliases = ["languages"]
```

## CarroTag
```toml
[myfile1]
hash = "644bcc7e564373040999aac89e7622f3ca71fba1d972fd94a31c3bfbf24e3938"
tags = ["rust", "coding"]
version = "1.0.0" # What version of CarroTag this was created to be compatible with 

[mydirectory]
# There is no hash for directories, just confirming name
tags = ["art", "painting", "fireworks"]
version = "1.0.0" # What version of CarroTag this was created to be compatible with 

[mydirectory.timing]
creation_time = "18-May-2025"
alternate_times = ["25-May-2025", "01-Jan-2100"]
```