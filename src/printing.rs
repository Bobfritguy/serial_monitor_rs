// Print Serial Data

pub(crate) fn print_serial_data(data: &[u8]) {
    let data = String::from_utf8_lossy(data);

    // Find all special characters, remove them and use coloured to print to screen.
    // [0m - Reset all attributes (normal text)
    // [1m - Bold text
    // [4m - Underline text
    // [31m - Red text
    // [32m - Green text
    // [33m - Yellow text
    // [34m - Blue text
    // [35m - Magenta text
    // [36m - Cyan text
    // [37m - White text
    // [90m to [97m - Bright versions of the above colors


    
    print!("{}", data);
}