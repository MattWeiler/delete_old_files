# delete_old_files
Delete old files via a command line interface.

## Reason for the application
I built this application because I was having an issue in which my **Plex Server**, running on **Windows 10**, was filling-up my ram-disk with transcoding files. Sometimes it would clear the files and sometimes it would not.

I tried to find a quick and easy way to delete those files on a regular schedule, but I wasn't able to, in 5 minutes, get it to work in a quiet no confirmation required way, so I figured I'd just write an application to do it.

### Why Rust
I wanted to play around with Rust and now I had a good reason :)

## Building & Running
I am clearly no Rust expert, I just built this application for fun Googling what I couldn't figure out.

Take my below steps with a grain of salt.

### Building
To build the application, just run the below command:

`cargo build`

*You can find your binary executable in the `target/debug` directory.*

### Building for Release
To build the application for release, just run the below command:

`cargo build --release`

*You can find your binary executable in the `target/release` directory.*

This binary file is completely stand-alone and should not require any external libraries to function.

### Running Locally
To run the application locally, just run the below command:

`cargo run`

*This command is a shorthand to build and execute the application.*

Running the above command as-is, you will be prompted with the help documentation.

If you want to apply some command-line arguments to the run command, you can do so by adding ` -- ` after `run` but before your arguments:

`cargo run -- -m 60 -d -p "C:\someDir"`
