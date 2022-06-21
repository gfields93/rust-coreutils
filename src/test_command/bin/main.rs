

use clap::Parser;
#[allow(non_snake_case)]
#[derive(Debug, Parser)]
struct CLI {
    ///‘-b file’ True if file exists and is a block special device.
    #[clap(short, value_name("file"), exclusive(true))]
    b: Option<bool>,
    ///‘-c file’ True if file exists and is a character special device.
    #[clap(short, value_name("file"), exclusive(true))]
    c: Option<bool>,
    ///‘-d file’ True if file exists and is a directory.
    #[clap(short, value_name("file"), exclusive(true))]
    d: Option<bool>,
    ///‘-f file’ True if file exists and is a regular file.
    #[clap(short, value_name("file"), exclusive(true))]
    f: Option<bool>,
    /**‘-h file’
    ‘-L file’ True if file exists and is a symbolic link. Unlike all other file-related tests, this
    test does not dereference file if it is a symbolic link. */
    #[clap(short, visible_short_alias('L'), value_name("file"), exclusive(true))]
    h: Option<bool>,
    ///‘-p file’ True if file exists and is a named pipe.
    #[clap(short, value_name("file"), exclusive(true))]
    p: Option<bool>,
    #[clap(short = 'S', value_name("file"), exclusive(true))]
    ///‘-S file’ True if file exists and is a socket.
    s_capital: Option<bool>,
    ///‘-t fd’ True if fd is a file descriptor that is associated with a terminal.
    #[clap(short, value_name("fd"), exclusive(true))]
    t: Option<bool>,

    //These options test for particular access permissions.
    ///‘-g file’ True if file exists and has its set-group-ID bit set.
    #[clap(short = 'g', value_name("file"), exclusive(true))]
    g: Option<bool>,
    ///‘-k file’ True if file exists and has its sticky bit set.
    #[clap(short, value_name("file"), exclusive(true))]
    k: Option<bool>,
    ///‘-r file’ True if file exists and the user has read access.
    #[clap(short, value_name("file"), exclusive(true))]
    r: Option<bool>,
    ///‘-u file’ True if file exists and has its set-user-ID bit set.
    #[clap(short, value_name("file"), exclusive(true))]
    u: Option<bool>,
    ///‘-w file’ True if file exists and the user has write access.
    #[clap(short, value_name("file"), exclusive(true))]
    w: Option<bool>,
    /**‘-x file’ True if file exists and the user has execute access (or search permission, if it is
    a directory).*/
    #[clap(short, value_name("file"), exclusive(true))]
    x: Option<bool>,
    ///‘-O file’ True if file exists and is owned by the current effective user ID.
    #[clap(short, value_name("file"), exclusive(true))]
    O: Option<String>,
    ///‘-G file’ True if file exists and is owned by the current effective group ID.
    #[clap(short = 'G', value_name("file"), exclusive(true))]
    g_capital: Option<bool>,

    ///‘-e file’ True if file exists.
    #[clap(short, value_name("file"), exclusive(true))]
    e: Option<bool>,
    ///‘-s file’ True if file exists and has a size greater than zero.
    #[clap(short = 's', value_name("file"), exclusive(true))]
    s: Option<bool>,

    ///‘-N file’ True if file exists and has been modified (mtime) since it was last read (atime).
    #[clap(short = 'N', value_name("file"), exclusive(true))]
    n_capital: Option<bool>,

    /// '-z file'True if the length of string is zero.
    #[clap(short, value_name("file"), exclusive(true))]
    z: Option<bool>,

    /**‘-n string’
     * 'string' True if the length of string is nonzero.
     */
    #[clap(short, value_name("string"), exclusive(true))]
    n: Option<bool>, 


    //Todo Implementation: Figure out how to position the arguments 

    #[clap(value_parser, index(1), required(true))]
    arg1: String,
    #[clap(value_parser(VALID_OPERATORS), index(2), requires("arg2"))]
    operator: Option<String>,
    #[clap(value_parser, index(3))]
    arg2: Option<String>
}

const VALID_OPERATORS: [&str; 11] = [
    "-nt",
    "-ot",
    "-ef",
    "-eq",
    "-ne",
    "-lt",
    "-gt",
    "-ge",
    "=",
    "==",
    "!="
];

fn main () {
    let _cli = CLI::parse();
}
