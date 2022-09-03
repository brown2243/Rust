Date : 2022-08-28 23:44

# Rust_PS

---

## 8 kyu

### Convert boolean values to strings 'Yes' or 'No'.

```rust
fn bool_to_word(value: bool) -> &'static str {
	match value {
	    true => "Yes",
	    false => "No",
    }
}
```

### DNA to RNA Conversion

```rust
fn dna_to_rna(dna: &str) -> String {
    let n = dna.len();
    let mut ans = String::new();
    let mut idx = 0;

    loop {
        if (idx == n) {
            break;
        }
        let t = dna.chars().nth(idx).unwrap();

        match t {
            'G' => ans.push_str("G"),
            'C' => ans.push_str("C"),
            'A' => ans.push_str("A"),
            _ => ans.push_str("U"),
        }

        idx += 1
    }
    return ans;
}
//
fn dna_to_rna(dna: &str) -> String {
 dna.replace("T", "U")
}
fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(char_conversion).collect()
}
fn char_conversion(c: char) -> char {
    if c == 'T' {
        return 'U';
    }

    c
}
fn dna_to_rna(dna: &str) -> String {
    let mut res = String::new();
    for s in dna.chars() {
        match s {
            'T' => res.push('U'),
            _ => res.push(s),
        }
    }
    res
}

```

### Counting sheep...

```rust
fn count_sheep(sheep: &[bool]) -> u8 {
    let mut cnt = 0;
    for x in sheep {
        if *x {
            cnt += 1;
        } else {
            cnt += 0
        }
    }
    cnt
}
//
fn count_sheep(sheep: &[bool]) -> u8 {
  sheep              // take the sheep array
    .iter()          // turn it into an iterable
    .filter(|&&x| x) // filter it by taking the values in the array and returning only the true ones
    .count() as u8   // count all of the elements in the filtered array and return a u8
}

```

### Fake Binary

```rust
fn fake_bin(s: &str) -> String {
    let mut ans = String::new();

    for x in s.trim().split("").into_iter() {
        if x == "" {
            continue;
        }
        let num = x
            .parse::<i32>()
            .expect("please give me correct string number!");

        // println!("{num}");
        if num >= 5 {
            ans.push_str("1");
        } else {
            ans.push_str("0");
        }
    }
    println!("{ans}");
    ans
}

//
fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c < '5' {'0'} else {'1'}).collect()
}
fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0'..='4' => '0',
            '5'..='9' => '1',
            _ => c
        })
        .collect()
}
```

### Switch it Up!

```rust
fn switch_it_up(n: usize) -> &'static str {
    match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "",
    }
}
//
fn switch_it_up(n: usize) -> &'static str {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "Zero"
    }
}
fn switch_it_up(n: usize) -> &'static str {
    match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => panic!()
    }
}

```

### The Feast of Many Beasts

```rust
fn feast(beast: &str, dish: &str) -> bool {
    return beast.chars().nth(0) == dish.chars().nth(0)
        && beast.chars().nth(beast.len() - 1) == dish.chars().nth(dish.len() - 1);
}
//
fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next()
    && beast.chars().last() == dish.chars().last()
}
fn feast(beast: &str, dish: &str) -> bool {
    dish[..1] == beast[..1] && dish[dish.len()-1..] == beast[beast.len()-1..]
}
```

### Function 2 - squaring an argument

```rust
fn square(n: i32) -> i32 {
    n * n
}
//
fn square(n: i32) -> i32 {
    n.pow(2)
}
```

### Convert number to reversed array of digits

```rust
// error
// Creates a temporary which is freed while still in use Again slight_smile
let process = Command::new(location_test);
process.arg(address);

fn digitize(n: u64) -> Vec<u8> {
    const RADIX: u32 = 10;
    // your code here
    let str = n.to_string();
    let arr = str
        .chars()
        .rev()
        .map(|x| x.to_digit(RADIX).unwrap())
        .collect::<Vec<u32>>();

    let mut ans: Vec<u8> = [].to_vec();
    arr.into_iter()
        .for_each(|val| ans.push(val.try_into().unwrap()));
    println!("{ans:#?}");
    return ans;
}
// u32 -> u8로 변경하는 부분
fn digitize(n: u64) -> Vec<u8> {
    n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}
```
