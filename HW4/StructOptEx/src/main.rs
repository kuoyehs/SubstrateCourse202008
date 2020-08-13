use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt;

//let p = PathBuf::from("/the/head");
//let os_str = p.into_os_string();

#[derive(Debug, StructOpt)]
struct Opts {
    //#[structopt(parse(from_os_str))]
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    //outfile: Option<PathBuf>,
    outfile: PathBuf,
}

impl IntoIterator for Opts {
    type Item = PathBuf;
    type IntoIter = OptsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        OptsIntoIterator {
            opts: self,
            index: 0,
        }
    }
}

struct OptsIntoIterator {
    opts: Opts,
    index: usize,
}

impl Iterator for OptsIntoIterator {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        let result = match self.index {
            0 => self.opts.infile.clone(),
            1 => self.opts.outfile.clone(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl fmt::Display for Opts {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //let infileStr=&self.infile.into_os_string();
        let infileStr=&self.infile.clone().into_os_string();
        let outfileStr=&self.outfile.clone().into_os_string();
        //let infileStr1=infileStr.into_os_string();
        //let outfileStr=&self.outfile.into_os_string();
        //match self.infile {
        //  Some(inner) => let infileStr=inner.into_os_string(),
        //  None => let infileStr="None",
        //}
        //let infileStr=self.infile.unwrap().display();
        //let outfileStr=self.outfile.display().unwrap();
        write!(f, "{} , {}", infileStr.to_str().unwrap(), outfileStr.to_str().unwrap())
    }
}

fn main() {
    let opts = Opts::from_args();
    let opts1=opts;
    let opts2=&opts1;
    println!("Display: {}", opts2);
    println!("{:?}", opts2);
    for component in opts1 {
        println!("{:?}", component);
    }
}
