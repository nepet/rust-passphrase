use clap::load_yaml;
use clap::App;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

mod dict;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let dict = dict::new_dict(dict::Lang::EN);

    if let Some(matches) = matches.subcommand_matches("generate") {
        let nw = matches.value_of("num_words").unwrap_or("3");
        let sep = matches.value_of("separator").unwrap_or("-");
        let nt = matches.value_of("num_trailing_numbers").unwrap_or("0");
        let np = matches.value_of("num_passphrases").unwrap_or("3");

        let nw_p = nw.parse::<i32>();
        if !nw_p.is_ok() {
            println!("error: {}", nw_p.unwrap_err());
            return;
        }
        let nw_int = nw_p.unwrap();

        let nt_p = nt.parse::<i32>();
        if !nt_p.is_ok() {
            println!("error: {}", nt_p.unwrap_err());
            return;
        }
        let nt_int = nt_p.unwrap();

        let np_p = np.parse::<i32>();
        if !np_p.is_ok() {
            println!("error: {}", np_p.unwrap_err());
            return;
        }
        let mut n = np_p.unwrap();
        
        while n > 0 {
            let mut words = get_words(nw_int, &dict);
            let mut strats: Vec<&dyn Fn(Vec<String>) -> Vec<String>> = vec![];
            let s1 = with_seperator(sep.to_string());
            let s2 = append_nums(nt_int);
            strats.push(&s1);

            if nt_int > 0 {
                strats.push(&s2);
            }

            words = apply_all(words, strats);
            let mut result: String = "".to_string();
            for w in words {
                result += &w;
            }
            println!("{}", result);
            n -= 1;
        }
    }
}

fn get_words(n: i32, dict: &Box<dyn dict::Dict>) -> Vec<String> {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut passwd: Vec<String> = vec![];

    let mut c = 0;
    while c < n {
        let rn = rng.gen_range(0..dict.size());
        passwd.push(dict.word(rn).to_string());
        c += 1;
    }

    passwd
}

fn with_seperator(sep: String) -> Box<dyn Fn(Vec<String>) -> Vec<String>> {
    Box::new(move |words: Vec<String>| {
        let size = words.len();
        let mut result: Vec<String> = vec![];
        let mut c = 1;

        for w in words {
            result.push(w);
            if c < size {
                result.push(sep.clone());
            }
            c += 1;
        }

        result
    })
}

fn append_nums(n: i32) -> Box<dyn Fn(Vec<String>) -> Vec<String>> {
    Box::new(move |words: Vec<String>| {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut result: Vec<String> = words.clone();
        let mut c = n.clone();
        while c > 0 {
            let rn = rng.gen_range(0..9);
            result.push(rn.to_string());
            c -= 1;
        }
        result
    })
}

// fn with_alternating_full_uppercase() {}

fn apply_all<F>(words: Vec<String>, funcs: Vec<F>) -> Vec<String>
where
    F: Fn(Vec<String>) -> Vec<String>,
{
    let mut w = words;
    for f in funcs {
        w = f(w)
    }
    w
}
