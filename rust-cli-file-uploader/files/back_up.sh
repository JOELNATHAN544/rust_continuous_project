#!/bin/bash

# check if the directory exists
if [ ! -d "$1" ]; then
    echo "Error: Directory '$1' does not exist."
    exit 1
fi

# archive the directory
archive="$1.tar"
tar -cvf "$archive" "$1/"

# compress the archive
gzip "$archive"

# create the backup directory if it doesn't exist
backup="backup_folder"
if [ ! -d "$backup" ]; then
    mkdir -p "$backup"
fi

# move the compressed archive to the backup directory
mv "$archive.gz" "$backup"

# store the log in a file
log="$backup/backup.log"
echo "Backup of directory $1 completed at $(date)" >>"$log"

# log the directory name to file1.txt
echo "$1" >file1.txt