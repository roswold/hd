fn main()
{
    let argv:Vec<String>=std::env::args().collect();
    let mut columns:usize=8;
    let mut files:Vec<String>=vec!();
    let mut curarg=1;
    let mut showoffset=true;
    let mut showascii=true;
    let helpmsg=format!("usage: {} [-ahn] [-w WIDTH] [--help] FILES\n\
                         -a          Don't display ASCII dump\n\
                         -h, --help  Display this help\n\
                         -n          Don't display binary file offset\n\
                         -w WIDTH    Specify number columns for output",
                        argv[0]);

    // Parse each file
    while curarg < argv.len()
    {

        // Set width of columns
        if argv[curarg]=="-w"
        {
            if argv.len()>curarg+1
            {
                curarg+=1;
                columns=argv[curarg].parse::<usize>().unwrap();
            }
            else
            {
                print!("error: -w requires WIDTH argument\n");
                std::process::exit(1);
            }
        }

        // Options
        else if argv[curarg].chars().nth(0).unwrap()=='-'
        {

            // Long option
            if argv[curarg].len()>curarg && argv[curarg].chars().nth(1).unwrap()=='-'
            {
                match argv[curarg].as_str()
                {

                    // Display help message
                    "--help" =>
                    {
                        print!("{}\n",helpmsg);
                        std::process::exit(0);
                    },

                    // Unrecognized option
                    _ =>
                    {
                        print!("error: unrecognized long option '{}'\n",argv[curarg]);
                        std::process::exit(1);
                    },
                }
            }

            // Short options
            else
            {
                for i in 1..argv[curarg].len()
                {
                    let c=argv[curarg].chars().nth(i).unwrap();

                    match c
                    {

                        // Toggle showoffset (display file offset of hexdump)
                        'n' =>
                        {
                            showoffset=false;
                        },

                        // Toggle showascii (show ascii rendering)
                        'a' =>
                        {
                            showascii=false;
                        },

                        // Display help message
                        'h' =>
                        {
                            print!("{}\n",helpmsg);
                            std::process::exit(0);
                        },

                        // Warning
                        'w' =>
                        {
                            print!("warning: use -w on its own\n");
                        },

                        // Unrecognized option
                        _ =>
                        {
                            print!("error: unrecognized short option '{}'\n",c);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }

        // Default options treated as input files
        else
        {
            files.push(argv[curarg].clone());
        }
        curarg+=1;
    }

    // Hexdump each argument previously
    // identified as a filename
    let l=files.len();
    for file in files
    {
        if l>1usize
        {
            print!("{}:\n",file);
        }
        hexdump(&file,columns,showoffset,showascii);
    }
}

// Hexdump a file
fn hexdump(filename:&String,columns:usize,showoffset:bool,showascii:bool)
{
    match std::fs::File::open(filename)
    {
        Ok(_) => {},
        Err(_) =>
        {
            print!("error: cannot open '{}'\n",filename);
            return;
        },
    }

    let data=&std::fs::read(filename).unwrap();

    let mut i:usize=0;
    while i<data.len()
    {

        let left=columns as i32-(data.len() as i32-i as i32);

        if showoffset
        {
            print!("{:08X}: ",i);
        }

        // Hexdump
        for j in 0..columns
        {
            if i+j>=data.len() {break;}
            print!("{:02X} ",data[i+j] as u32);
        }

        if showascii
        {
            // ASCII
            for _ in 0..left
            {
                print!("   ");
            }

            for j in 0..columns
            {
                //i+=1;
                if i+j>=data.len() {break;}
                if (data[i+j] as u32 > 32) && (data[i+j] as u32) < 128
                {
                    print!("{}",data[i+j] as char);
                }
                else
                {
                    print!(".");
                }
            }
        }

        print!("\n");
        i+=columns;
    }
    print!("\n");
}
