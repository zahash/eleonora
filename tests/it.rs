use std::time::{SystemTime, UNIX_EPOCH};

use eleonora::ThreadPool;

fn expensive_greet(name: String) -> String {
    let mut a: usize = 0;
    for _ in 1..300_000_000 as usize {
        a += 1;
        a -= 1;
    }

    format!("Hello, {name}")
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[test]
fn test() {
    let words: Vec<String> = vec![
        "dog".into(),
        "cat".into(),
        "rat".into(),
        "fat".into(),
        "mouse".into(),
        "tint".into(),
        "asdf".into(),
        "qwer".into(),
        "1234".into(),
        "dog".into(),
        "cat".into(),
        "rat".into(),
    ];

    let words_once = words[0].clone();
    let once = || {
        let output = expensive_greet(words_once);
        // println!("{}", output);
    };

    let words_seq = words.clone();
    let seq = || {
        let output = words_seq
            .into_iter()
            .map(|word| expensive_greet(word))
            .collect::<Vec<String>>();

        // println!("{:?}", output);
    };

    let words_par = words.clone();
    let par = || {
        let pool = ThreadPool::new(None);

        for word in words_par {
            pool.execute(|| expensive_greet(word))
        }

        let output = pool.outputs().into_iter().collect::<Vec<String>>();
        // println!("{:?}", output);
    };

    let start = now();
    // once();
    // seq();
    par();
    let end = now();

    println!("time elapsed in secs: {}", end - start);
}
