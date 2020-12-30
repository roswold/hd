struct Hdinfo
{
    columns:usize,
    showoffset:bool,
    showascii:bool,
    numbytes:usize,
    percent:f32,
}

fn newhd() -> Hdinfo
{
    return Hdinfo{
        columns:8,
        showoffset:true,
        showascii:true,
        numbytes:0,
        percent:1.0,
    }
}

fn main()
{
    let argv:Vec<String>=std::env::args().collect();
    let mut files:Vec<String>=vec!();
    let mut curarg=1;
    let mut h:Hdinfo=newhd();
    let helpmsg=format!("usage: {} [-abh] [-n OFFSET] [-w WIDTH] [--help] FILES\n\
                         -a          Don't display ASCII dump\n\
                         -b          Don't display binary file offset\n\
                         -h, --help  Display this help\n\
                         -n SIZE     Number of bytes to read\n\
                         -w WIDTH    Specify number of columns for output\n\n\
                         SIZE        Either an unsigned integer or a percentage such as 50%",
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
                h.columns=match argv[curarg].parse::<usize>()
                {
                    Ok(n) => n,
                    Err(_) => 8,
                };
            }
            else
            {
                print!("error: -w requires WIDTH argument\n");
                std::process::exit(1);
            }
        }

        // Set number of bytes to read
        else if argv[curarg]=="-n"
        {
            if argv.len()>curarg+1
            {
                curarg+=1;

                // Set percentage
                if argv[curarg].chars().nth(argv[curarg].len()-1).unwrap()=='%'
                {
                    h.percent=match argv[curarg][0..argv[curarg].len()-1].parse::<usize>()
                    {
                        Ok(n) => n as f32/100.0,
                        Err(_) => 1.0,
                    };
                }

                // Set bytes offset
                else
                {
                    h.numbytes=match argv[curarg].parse::<usize>()
                    {
                        Ok(n) => n,
                        Err(_) => 0,
                    };
                }
            }

            else
            {
                print!("error: -n requires WIDTH argument\n");
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
                        'b' =>
                        {
                            h.showoffset=false;
                        },

                        // Toggle showascii (show ascii rendering)
                        'a' =>
                        {
                            h.showascii=false;
                        },

                        // Display help message
                        'h' =>
                        {
                            print!("{}\n",helpmsg);
                            std::process::exit(0);
                        },

                        // Warning for short options with arguments
                        'w' | 'n' =>
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
        hexdump(&file,&h);
    }
}

// Hexdump a file
fn hexdump(filename:&String,h:&Hdinfo)
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
    let mut length:usize=data.len();
    let mut i:usize=0;

    // Set length appropriately
    if h.numbytes>0 && h.numbytes<data.len()
    {
        length=h.numbytes;
    }

    else if h.percent<1.0
    {
        length=(data.len() as f32*h.percent) as usize;
    }

    while i<length
    {
        let left=h.columns as i32-(length as i32-i as i32);

        if h.showoffset
        {
            print!("{:08X}: ",i);
        }

        // Hexdump
        for j in 0..h.columns
        {
            if i+j>=length {break;}
            print!("{:02X} ",data[i+j] as u32);
        }

        if h.showascii
        {
            // ASCII
            for _ in 0..left
            {
                print!("   ");
            }

            for j in 0..h.columns
            {
                //i+=1;
                if i+j>=length {break;}
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
        i+=h.columns;
    }
    print!("\n");
}
