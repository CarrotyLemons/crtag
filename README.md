# Problem statement
Currently I am running into problems with my file system. When my files have multiple attributes (e.g. something is both a rust program and a control algorithm). I am unable to hierarchically organise my files without losing information. I want to be able to tag files, with similiar functionality to TagStudio. 
- Can add attributes to attributes
- Able to search files by tag
    - name
    - alias
    - shorthand
- Tag metadata is bundled along with files
- Absolutely must be system interoperable (CLI should work in any bash shell)
But TagStudio is missing some features I want
- CLI
- Information to be stored in sidecar files (`CarroTag.toml`?), nothing lost if CarroTag software instance is erased.
- On CarroTag startup all directories search and internal state is processed. Changes made are pushed out to the filesystem (error raised if conflict found)
- Human readable information storage format (intersystem operability when no CarroTag available)
- App created through Rust
- This allows a very fast CarroTag interface while preserving human readability (internal state is in rust program but saved to external state `CarroTag.toml` files)
- Greater focus towards all files (tagging for directories and any file type)

This would allow me to structure folders up until I hit that attribute issue, at which point all further files and directories are organised flat and held at the same level in the same directory. At this point a metadata file would take over and be read by CarroTag.