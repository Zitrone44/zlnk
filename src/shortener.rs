use env_loader::Env;
use rand::{thread_rng, Rng};
use rocksdb::DB;

pub fn random(length: usize) -> String {
    let mut rng = thread_rng();
    let rands = rng.gen_iter::<usize>().take(length).collect::<Vec<usize>>();
    let hex = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    let mut result = vec![];
    for rand in rands {
        let num = rand % 16;
        result.push(hex[num]);
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
                let random_value = random(env.short_length);
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