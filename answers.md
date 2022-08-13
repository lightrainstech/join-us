# About you.
1. I'm Daksh, known as `weegee` online, I'm a rust developer since more than 2 years now and professionaly for 1.5 years now, I like to work on open source in my free time
 or play games or talk on discord. I'm currently doing my bachelors at Penn State.
2. I own a Macbook Air (M1) 16 gb
3. I like to be lightweight and fast here to increase my productivity, so I use sublime with LSP and rust analyser. AFAIK Sublime text is [just the fastest editor out there](https://www.youtube.com/watch?v=_mZOzyzlsww&ab_channel=simplysublime). Macos with Alacritty as my terminal, zsh as my shell with a highly customized config and theme, sublime merge for git.

# Social Profile
1. [Stackoverflow](https://stackoverflow.com/users/10967889/weegee)
2. [Personal website](https://dakshu.xyz/), [recent blog I wrote](https://dakshu.xyz/blog/dpmucl.html) and this [cool project I'm working on ](https://github.com/Daksh14/croc-look)

# The real stuff.
1. `brew list` says: go, ruby, sass, dart. Rust with rustup, nightly and beta 1.49.0 and lastest nightly, beta and stable
2. [Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf3b134992b78ae2b614aa0166338702)
```rust
fn num_to_array(mut num: usize) -> Vec<usize> {
    let mut answer = Vec::new();
    while num > 9 {
        answer.push(num.rem_euclid(10));
        num = num.wrapping_div(10);
    }
    answer.push(num);
    answer.reverse();
    answer
}
```
3. This assumes the array is sorted. [Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=54ec63145b5e386d69ce2ad401be9ab9)
```rust
fn remove_duplicates<T: Clone + PartialEq>(vec: Vec<T>) -> Vec<T> {
    vec.iter()
        .enumerate()
        .filter(|(index, value)| Some(value) != vec.get(index.wrapping_sub(1)).as_ref())
        .map(|(_, val)| val.clone())
        .collect()
}
```
4. [Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=61b8b30adf619651aedee01043d710b4)
```rust
fn to_pig_latin<T: AsRef<str>>(string: T) -> String {
    let mut collected: String = string.as_ref().split(' ').map(|word| {
        // grab all the letters leaving the first
        let mut string = String::from(&word[1..word.len()]);
        // push the first character and lowercase it
        string.push_str(&word[0..1].to_lowercase().to_string().as_str());
        string.push_str("ay ");
        string
    }).collect();
    // remove the last space
    collected.remove(collected.len()-1);
    collected
}

fn to_normal<T: AsRef<str>>(string: T) -> String {
    let mut collected: String = string.as_ref().split(' ').map(|word| {
        let word_len = word.len();
        // grab the letter before ay
        let before_ay = &word[word_len-3..word_len-2];
        // convert the letter before ay to a string
        let mut string = String::from(before_ay);
        // push the rest before the first letter and ay to the string
        string.push_str(&word[0..word_len-3]);
        string.push(' ');
        string
    }).collect();
    // remove the last space
    collected.remove(collected.len()-1);
    collected
}
```
5. This doesn't create a copy of the list but reallocates `k` elements if shifting left by `k`. I needed two swap operations in code, total swap operations is `vec.len()`. This doesn't get too bad if `k` is small, doesn't matter how big the array is. [Playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=67202f60c7de2c5218091b0e631a3775)
```rust
fn rotate<'a, T: Clone>(mut vec: Vec<&'a T>, n: usize) -> Vec<&'a T> {
    let len = vec.len();
    // store the first n values
    let temp_store = Vec::from(&vec[0..n]);

    // ignore first n, shif the rest
    for i in 0..len - n {
        vec[i] = vec[i + n];
    }
    // push the rest
    for i in 0..n {
        vec[len - 1 - i] = temp_store[temp_store.len() - 1 - i]
    }

    vec
}
```
