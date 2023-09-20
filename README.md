# image-resizer

Given a directory path, this program will resize all images in the directory that are larger than 2MB to be ~2MB by scaling down their dimensions proportionally to the diff between their size and the max file size (2MB). If a directory arg is not given, the program will use the current directory after asking for confirmation.

- run .exe: ./image-resizer --path <path_to_directory>
