mod test;
mod conv;
mod fast_conv;
mod snippets;

use rustfft::num_complex::Complex;
use std::io::Write;
use std::path::PathBuf;
use fast_conv::fast_convolution;
use std::fs::File;

use std::fs;
use std::error::Error;
use clap::Parser;
use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    drypath: std::path::PathBuf,

    #[clap(parse(from_os_str))]
    corepath: std::path::PathBuf,

    #[clap(parse(from_os_str))]
    outputpath: std::path::PathBuf,
}


fn parse_vector(filename: PathBuf) -> Vec<Complex<f64>> {
    let name = filename.clone();
    let contents = fs::read_to_string(name).expect("Something went wrong reading the file");
    let regex = Regex::new(r"([ \s,.]+)").expect("Invalid regex");
    return regex.split(&contents).map(|s| Complex::new(s.parse::<f64>().unwrap(), 0.0)).collect::<Vec<Complex<f64>>>();
}

fn write(filename: PathBuf, data: Vec<Complex<f64>>) {
    let mut f = File::create(filename).expect("Unable to create file");  
    let strings: Vec<String> = data.iter().map(|n| n.to_string()).collect();
    writeln!(f, "{}", strings.join("\n")).expect("error writing an output");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let mut dry: Vec<Complex<f64>> = parse_vector(args.drypath);
    let mut core: Vec<Complex<f64>> = parse_vector(args.corepath);
    write(args.outputpath, fast_convolution(&mut dry,&mut core));
    Ok(())
}



// fn main() {
//     let args: Vec<String> = env::args().collect();

//     println!("In file {}", args[0]);

//     let mut file = File::open(args[0].clone()).unwrap();
//     let mut stdout = stdout();
//     let mut str = &copy(&mut file, &mut stdout).unwrap().to_string();
//     let data = Json::from_str(str).unwrap();

    // println!("With text:\n{}", str);
    // let mut v1 = generate_mock_buffer(0.5, 5000, 0.0, FFT_EFFECTIVE_SIZE);
    // let mut v1_1 = v1.to_vec();
    // let mut v2 = generate_mock_buffer(0.5, 3000, 0.0, FFT_EFFECTIVE_SIZE);
    // let mut v2_1 = v2.to_vec(); 

    // let direct_conv_time = Instant::now();
    // let mut direct_conv = convolution(&v1, &v2);
    // let direct_dur = direct_conv_time.elapsed();
    // println!("direct convolution {:?}", direct_dur);

    // let fast_conv_time = Instant::now();
    // let mut fast_conv = fast_convolution(&mut v1_1, &mut v2_1);
    // let fast_dur = fast_conv_time.elapsed();
    // println!("fast convolution {:?}", fast_dur);

    


    // println!("direct");
    // for i in direct_conv.iter() {
    //     println!("{}", i)
    // }

    // println!("fast");
    // for i in fast_conv.iter() {
    //     println!("{}", i)
    // }
// }
