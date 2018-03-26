use env_loader::Env;
use rand::{thread_rng, Rng};
use rocksdb::DB;

pub fn random(length: usize, alphabet_name: String) -> String {
    let mut rng = thread_rng();
    let rands = rng.gen_iter::<usize>().take(length).collect::<Vec<usize>>();
    let alphabet;
    if alphabet_name == "hex" {
        alphabet = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    } else if alphabet_name == "decimal" {
        alphabet = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    } else if alphabet_name == "alpha" {
        alphabet = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]
    } else if alphabet_name == "alpha-numeric" {
        alphabet = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
    } else {
        panic!("Invalid Alphabet");
    }
    let mut result = vec![];
    for rand in rands {
        let num = rand % alphabet.len();
        result.push(alphabet[num]);
    }
    result.join("")
}

pub fn short(long_url: String, env: &Env, database: &DB) -> Option<String> {
    let valid = env.url_regex.is_match(&long_url);
    if valid {
        let existing = database.get(long_url.as_bytes()).unwrap();
        match existing {
            Some(existing_short) => {
                Some(String::from_utf8(existing_short.to_vec()).unwrap())
            }
            None => {
                let random_value = random(env.short_length, env.short_alphabet.to_owned());
                database.put(random_value.as_bytes(), long_url.as_bytes()).unwrap();
                database.put(long_url.as_bytes(), random_value.as_bytes()).unwrap();
                Some(random_value)
            }
        }
    } else {
        None
    }
}

pub fn long(short_url: String, database: &DB) -> Option<String> {
    let long = database.get(short_url.as_bytes()).unwrap();
    match long {
        Some(bytes) => {
            Some(String::from_utf8(bytes.to_vec()).unwrap())
        }
        None => {
            None
        }
    }

}