/*

The following reserved characters:

< (less than)
> (greater than)
: (colon)
" (double quote)
/ (forward slash)
\ (backslash)
| (vertical bar or pipe)
? (question mark)
* (asterisk)
Integer value zero, sometimes referred to as the ASCII NUL character.

Characters whose integer representations are in the range from 1 through 31, except for alternate data streams where these characters are allowed. For more information about file streams, see File Streams.



Any other character that the target file system does not allow.
*/

use regex::Regex;
pub fn get_file_name(filename: String, org_filename: String) -> String {
    let mut filename = filename.replace(
        &[
            '(', ')', ',', '\"', '.', ';', ':', '\'', ' ', '*', '!', '?', '<', '>', '_',
        ][..],
        "_",
    );
    let a = format!("{}.{}", filename, org_filename);
    return a;
}
