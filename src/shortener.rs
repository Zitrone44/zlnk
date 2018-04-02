use env_loader::Env;
use stats::Stats;
use rand::{thread_rng, Rng};
use r2d2;
use r2d2_redis;
use redis::Commands;

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

pub fn short(long_url: String, env: &Env, connection: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, retries: Option<u8>) -> Option<String> {
    let valid = env.url_regex.is_match(&long_url);
    if valid {
        let existing = connection.get(format!("long_{}", &long_url));
        match existing {
            Ok(existing_short) => {
                Some(existing_short)
            }
            Err(_err) => {
                let random_value = random(env.short_length, env.short_alphabet.to_owned());
                let exist = connection.get(format!("short_{}", &random_value));
                match exist {
                    Ok(_short) => {
                        let _: String = _short;
                        let retry = retries.unwrap_or(0);
                        if retry == 10 {
                            None
                        } else {
                            short(long_url, env, connection, Some(retry + 1))
                        }
                    }
                    Err(_err) => {
                        let _: () = connection.set(format!("short_{}", &random_value), &long_url).unwrap();
                        let _: () = connection.set(format!("long_{}", &long_url), &random_value).unwrap();
                        Some(random_value)
                    }
                }
            }
        }
    } else {
        None
    }
}

pub fn long(short_url: String, env: &Env, connection: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>, stats: Stats) -> Option<String> {
    let long = connection.get(format!("short_{}", &short_url));
    match long {
        Ok(long) => {
            if !env.disable_stats {
                stats.save(short_url, connection).unwrap();
            }
            Some(long)
        }
        Err(_err) => {
            None
        }
    }
}