## CP-Tracker
Small utility script I wrote for maintaining the log for this repo. It maintains the log in a sqlite file and can convert it into html/md and vice versa

## Build
You will need prettier installed from your package manager to use this
To build the binary, use
```
cargo build --release
```

## Usage
- To import data from a html or md file into the database
  ```
  cp-tracker import
  ```
  Note: It uses the file specified as INFILE in main.rs
- To add an entry to the log
  ```
  cp-tracker update
  ```
  A series of prompts will follow to enter the details for the entry
- To export data from the database to an html and md file
  ```
  cp-tracker import
  ```
  Note: It uses the file specified as OUTFILE in main.rs. Also the md file is prefixed with the PREFIX specified in main.rs
