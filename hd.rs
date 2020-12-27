// Hexdump a file
fn hexdump(filename:&String,columns:usize)
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
    //let data_str=std::str::from_utf8(data).unwrap();

    print!("{}:\n",filename);
    let mut i:usize=0;
    while i<data.len()
    {

        let left=columns as i32-(data.len() as i32-i as i32);

        // Hexdump
        for j in 0..columns
        {
            if i+j>=data.len() {break;}
            print!("{:02x} ",data[i+j] as u32);
        }

        // ASCII
        for _ in 0..left
        {
            print!("   ");
        }

        for j in 0..columns
        {
            //i+=1;
            if i+j>=data.len() {break;}
            if data[i+j] as u32 > 32
            {
                print!("{}",data[i+j] as char);
            }
            else
            {
                print!(".");
            }
        }

        print!("\n");
        i+=columns;
    }
    print!("\n");
}

fn main()
{
    let argv:Vec<String>=std::env::args().collect();
    let mut columns:usize=8;
    let mut files:Vec<String>=vec!();
    let mut curarg=1;

    // Parse each file
    while curarg < argv.len()
    {

        // Parse each argument
        match (&argv[curarg]).as_str()
        {

            // Display help message
            "--help"|"-h" =>
            {
                print!("usage: {} [-w WIDTH] [--help|-h] FILES\n",
                        argv[0]);
                std::process::exit(0);
            },

            // Set width/number of columns
            "-w" =>
            {
                if argv.len()<=curarg+1
                {
                    print!("error: -w: width required\n");
                    std::process::exit(1);
                }

                columns=match argv[curarg+1].parse::<usize>()
                {
                    Ok(n) => n,
                    Err(_) => {print!("error: -w: invalid width argument\n"); 8 as usize},
                };

                if columns<2 || columns > 128
                {
                    columns=8;
                }
                curarg+=1;
            },

            // Treat default arguments as filenames
            //_ => {hexdump(&argv[curarg],columns);},
            _ => {files.push(argv[curarg].clone());},
        }
        curarg+=1;
    }

    // Hexdump each argument previously
    // identified as a filename
    for file in files
    {
        hexdump(&file,columns);
    }
}
