use home::home_dir;
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct DicewareGen {
    words: Vec<String>,
    rng: ThreadRng,
}

// this assumes that we have already the file downloaded here
impl DicewareGen {
    pub fn new() -> Result<DicewareGen, Box<dyn Error>> {
        let dir = if let Some(home_path) = home_dir() {
            String::from(format!(
                "{}/.rustyvault/wordlists",
                home_path.to_string_lossy()
            ))
        } else {
            String::from("/.rustyvault/wordlists")
        };
        // get dir in .rustyvault
        let p = Path::new(&dir).join("eff_large_wordlist.txt");
        let f = File::open(p)?;

        Ok(DicewareGen {
            words: {
                BufReader::new(f)
                    .lines()
                    .map(|l| l.expect("Error parsing wordlist"))
                    .collect()
            },
            rng: thread_rng(),
        })
    }

    pub fn gen(mut self, rounds: u8) -> Vec<String> {
        (0..rounds)
            .map(|_| {
                let x = self.rng.gen_range(0..7775);
                self.words[x].clone()
            })
            .collect()
    }
}
