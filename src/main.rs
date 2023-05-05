use std::env;
use std::fs;
use std::fs::DirEntry;
use std::io::Result;
use std::ops::Sub;
use std::path::Path;
use std::process::exit;
use std::time::{ SystemTime, UNIX_EPOCH };

///////////////
// CONSTANTS //
///////////////
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const ARGUMENT_FLAG_PREFIX: &str = "-";
const ARGUMENT_FLAG_HELP1: &str = "-h";
const ARGUMENT_FLAG_HELP2: &str = "-help";
const ARGUMENT_FLAG_HELP3: &str = "-?";
const ARGUMENT_FLAG_HELP4: &str = "/?";
const ARGUMENT_FLAG_PATH: &str = "-p";
const ARGUMENT_FLAG_MINS: &str = "-m";
const ARGUMENT_FLAG_DELETION: &str = "-d";

///////////////
// FUNCTIONS //
///////////////
fn main() {
    // Define the variables to hold the specified inputs.
    let mut root_dir_path = String::new();
    let mut min_file_age_mins = 60;
    let mut perform_deletes = false;

    let args: Vec<_> = env::args().collect();
    let mut current_flag: String = String::new();

    // Loop over all of the command-line arguments and extract
    // the expected values.
    for arg in args
    {
        // println!("Argument: {arg}");

        // If the current argument is any of the HELP flags,
        // then print the help documentation and exit with a
        // code of (0).
        if  arg.eq_ignore_ascii_case(ARGUMENT_FLAG_HELP1)
                ||
            arg.eq_ignore_ascii_case(ARGUMENT_FLAG_HELP2)
                ||
            arg.eq_ignore_ascii_case(ARGUMENT_FLAG_HELP3)
                ||
            arg.eq_ignore_ascii_case(ARGUMENT_FLAG_HELP4)
        {
            print_help();

            exit(0);
        }
        // If the current argument is the DELETION flag, then
        // set the perform_deletes variable.
        else if arg.eq_ignore_ascii_case(ARGUMENT_FLAG_DELETION)
        {
            perform_deletes = true;
        }
        // If the current argument starts with the common flag
        // prefix, then set the current_flag variable to the
        // current argument.
        // It is expected that the next argument will be the
        // value for the current argument flag.
        else if arg.starts_with(ARGUMENT_FLAG_PREFIX)
        {
            current_flag = String::new();
            current_flag.push_str(&arg);
        }
        // If the current_flag variable is empty, then skip the
        // current argument as it's not something that we're
        // expecting.
        else if current_flag.is_empty()
        {
            continue;
        }
        // If the current_flag variable is for that of the
        // root-dir-path, then set the root_dir_path variable to
        // the value of the current argument.
        else if current_flag.eq_ignore_ascii_case(ARGUMENT_FLAG_PATH)
        {
            root_dir_path = String::new();
            root_dir_path.push_str(&arg);
        }
        // If the current_flag variable is for that of the
        // min-file-age, then set the min_file_age_mins variable to
        // the value of the current argument (parsed as u32).
        else if current_flag.eq_ignore_ascii_case(ARGUMENT_FLAG_MINS)
        {
            min_file_age_mins = arg.parse().unwrap();
        }
    }

    // If no root-dir-path was specified, then print an error followed by
    // the help documentation and then exit with a code of (1).
    if root_dir_path.len() <= 0
    {
        println!("[ERROR] A root path must be specified.");
        println!();

        print_help();

        exit(1);
    }

    // Print out the specified inputs.
    println!();
    println!("Root path: {root_dir_path}");
    println!("Min file age : {min_file_age_mins} mins");
    println!("Delete Mode: {perform_deletes}");
    println!();

    // Call the function to actually delete the contents of the specified
    // root-dir-path.
    match delete_directory_contents(
        root_dir_path,
        min_file_age_mins,
        perform_deletes
    ) {
        Ok(all_deleted) => println!("All files deleted: {all_deleted}"),
        Err(_) => println!("An error occurred while deleting files."),
    };

    exit(0);
}

/**
 * This function will print the help documentation to the console.
 */
fn print_help() {
    println!("NAME");
    println!("\tdelete_old_files");
    println!();

    println!("VERSION");
    println!("\t{APP_VERSION}");
    println!();

    println!("SYNOPSIS");
    println!("\tRecursively deletes all files and directories within the specified root directory that have not been modified for at least the specified min-file-age.");
    println!("\tChild directories will only be deleted if all files, and directories, within that child directory have been deleted.");
    println!();

    println!("SYNTAX");
    println!("\tdelete_old_files.exe [{ARGUMENT_FLAG_MINS} <uInt32>] [{ARGUMENT_FLAG_DELETION}] {ARGUMENT_FLAG_PATH} <String>");
    println!();
    println!("\t{ARGUMENT_FLAG_HELP1}\tAny of these flags denote that this help documentation should be printed.");
    println!("\t{ARGUMENT_FLAG_HELP2}\t");
    println!("\t{ARGUMENT_FLAG_HELP3}\t");
    println!("\t{ARGUMENT_FLAG_HELP4}\t");
    println!();
    println!("\t{ARGUMENT_FLAG_MINS}\tThis flag must be followed by a positive integer value to denote the minimum number of minutes that a file must not have been modified for, in order to be deleted.");
    println!();
    println!("\t{ARGUMENT_FLAG_DELETION}\tThis flag denotes if file deletions should actually take place.");
    println!("\t\tIf this is omited, then only the deletion messages will be printed and no files or directories will actually be deleted.");
    println!();
    println!("\t{ARGUMENT_FLAG_PATH}\tThis flag must be followed by the full path to the root directory from which any child files and directories are to be deleted.");
    println!();

    println!("EXAMPLES");
    println!("\tdelete_old_files.exe {ARGUMENT_FLAG_MINS} 120 {ARGUMENT_FLAG_DELETION} {ARGUMENT_FLAG_PATH} \"C:\\myDir\"");
    println!("\tThis will delete all files within the \"C:\\myDir\" directory that have not been modified for at least 120 minutes.");
    println!();
    println!("\tdelete_old_files.exe {ARGUMENT_FLAG_DELETION} {ARGUMENT_FLAG_PATH} \"C:\\myDir\"");
    println!("\tThis will delete all files within the \"C:\\myDir\" directory that have not been modified for at least 60 minutes (as 60 is the default value).");
    println!();
    println!("\tdelete_old_files.exe {ARGUMENT_FLAG_MINS} 120 {ARGUMENT_FLAG_PATH} \"C:\\myDir\"");
    println!("\tThis will print-out the deletion messages for, but not actually delete, all files within the \"C:\\myDir\" directory that have not been modified for at least 120 minutes.");
}

/**
 * This function will delete the specified directory along
 * with any children files/directories.
 * 
 * @param path
 * The path to the directory.
 * @param min_file_age_mins
 * The minimum number of minutes that a file must be old
 * before it can be deleted.
 * @param perform_deletes
 * The boolean flag to denote if the files should actually
 * be deleted or just the messages logged.
 */
fn delete_directory_contents<P: AsRef<Path>>(
    path: P,
    min_file_age_mins: u32,
    perform_deletes: bool
) -> Result<bool> {
    let mut all_deleted = true;

    for entry in fs::read_dir(path)? {
        let current_entry = entry?;
        let current_path = current_entry.path();
        let current_path_str = current_path.to_str().unwrap();
        let mut current_all_deleted = true;

        // If the current file is a directory, then recursively
        // delete it's contents.
        if current_entry.file_type()?.is_dir()
        {
            // Recursively delete all files/directories within
            // the current directory.
            current_all_deleted = delete_directory_contents(
                &current_path,
                min_file_age_mins,
                perform_deletes
            ).unwrap() && current_all_deleted;

            // If not all files/directories within the current
            // directory were deleted, then log an error and mark
            // that all files/directories were not deleted.
            if !current_all_deleted
            {
                println!("Failed to delete all files/directories within directory: {current_path_str}");

                all_deleted = false;

                continue;
            }

            // Since all files/directories within the current
            // directory were deleted, then delete the current
            // directory.
            let mut current_dir_deleted = true;

            // If deletions should be performed, then do it.
            if perform_deletes
            {
                current_dir_deleted = match fs::remove_dir(&current_path) {
                    Ok(()) => true,
                    Err(_) => false,
                };
            }

            // If deleting the current directory was successful, then log a
            // success message and continue the loop.
            if current_dir_deleted
            {
                println!("Directory deleted: {current_path_str}");

                continue;
            }

            // Since deleting the current directory was not successful, then log
            // an error and mark the all_deleted to false.
            println!("Failed to delete directory: {current_path_str}");

            all_deleted = false;
        }
        else
        {
            // If the current file is not old enough to be deleted, then
            // log an error, mark the all_deleted to false and continue
            // the loop.
            if !is_file_old_enough(current_entry, min_file_age_mins)
            {
                println!("File not old enough to be deleted: {current_path_str}");

                all_deleted = false;

                continue;
            }

            // Since the current file is old enough to be deleted, delete it.
            let mut current_file_deleted = true;

            // If deletions should be performed, then do it.
            if perform_deletes
            {
                current_file_deleted = match fs::remove_file(&current_path) {
                    Ok(()) => true,
                    Err(_) => false,
                };
            }

            // If deleting the current file was successful, then log a
            // success message and continue the loop.
            if current_file_deleted
            {
                println!("File deleted: {current_path_str}");

                continue;
            }

            // Since deleting the current file was not successful, then log
            // an error and mark the all_deleted to false.
            println!("Failed to delete file: {current_path_str}");

            all_deleted = false;
        }
    }
    Ok(all_deleted)
}

/**
 * This function will determine if the specified DirEntry
 * represents a file that is old enough to be deleted.
 * 
 * @param file_entry
 * The DirEntry of the file.
 * @param min_file_age_mins
 * The minimum number of minutes that a file must be old
 * before it can be deleted.
 */
fn is_file_old_enough(
    file_entry: DirEntry,
    min_file_age_mins: u32
) -> bool {
    // Extract the meta-data from the file.
    let meta_data = match file_entry.metadata() {
        Ok(meta_data) => meta_data,
        Err(_) => {
            return false;
        },
    };

    // Extract the last-modified-date from the meta-data.
    let last_modified = match meta_data.modified() {
        Ok(last_modified) => match last_modified.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(_) => {
                return false;
            },
        },
        Err(_) => {
            return false;
        },
    };

    // Extract the current system-time.
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => {
            return false;
        },
    };

    // Calculate the difference in time between the current
    // system time and the last-modified-date of the file.
    let time_diff = current_time.sub(last_modified);
    let time_diff_mins = time_diff.as_millis() / 60 / 1000;

    // let file_name = file_entry.file_name();
    // let file_name_str = file_name.to_str().unwrap();
    // println!("File age {time_diff_mins} mins: {file_name_str}");

    // Return the boolean flag to denote if the age of the file
    // is greater-than-or-equal-to the min-file-age_mins.
    return time_diff_mins >= (min_file_age_mins as u128);
}
