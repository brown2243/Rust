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

---

### Grasshopper - Messi Goals

```rust
static la_liga_goals: u32 = 43;
static champions_league_goals: u32 = 10;
static copa_del_rey_goals: u32 = 5;

static total_goals: u32 = la_liga_goals+champions_league_goals+copa_del_rey_goals;
```

### Remove First and Last Character

```rust
pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

```

### Welcome!

```rust
fn greet(language: &str) -> &str {
    match language {
        "czech" => "Vitejte",
        "danish" => "Velkomst",
        "dutch" => "Welkom",
        "estonian" => "Tere tulemast",
        "finnish" => "Tervetuloa",
        "flemish" => "Welgekomen",
        "french" => "Bienvenue",
        "german" => "Willkommen",
        "irish" => "Failte",
        "italian" => "Benvenuto",
        "latvian" => "Gaidits",
        "lithuanian" => "Laukiamas",
        "polish" => "Witamy",
        "spanish" => "Bienvenido",
        "swedish" => "Valkommen",
        "welsh" => "Croeso",
        _ => "Welcome",
    }
}
```

### Are You Playing Banjo?

- Rust에는 문자열 타입이 두가지 존재한다. 언어 자체로 지원하는 str과 표준 라이브러리에서 지원하는 String이 그렇다.
- `let s1: &str = "Hello str";`
- `let s2: String = String::from("Hello String");`
- str은 보통 &str로 많이 사용한다.
- String과 &str의 가장 큰 차이점은 String은 문자열 수정이 가능하지만 &str은 불가능하다는 점이다.
- &str은 보통 문자열 리터럴이나 문자열 슬라이스를 저장하는데 사용된다.
- 출처: https://steelbear.tistory.com/86 [steelbear's notes:티스토리]

```rust
fn are_you_playing_banjo(name: &str) -> String {
    let name = name.to_string();
    if name.starts_with('r') || name.starts_with('R') {
        name + " plays banjo"
    } else {
        name + " does not play banjo"
    }
}
//
fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}
fn are_you_playing_banjo(name: &str) -> String {
    match name.to_lowercase().starts_with("r") {
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name)
    }
}
```

## 7 kyu

### The highest profit wins!

<!--  * 를 아직 명확하게 모름 -->

```rust
fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}
//
use itertools::Itertools;

fn min_max(xs: &[i32]) -> (i32, i32) {
    xs.iter().cloned().minmax().into_option().unwrap()
}
fn min_max(lst: &[i32]) -> (i32, i32) {
    let min = lst.iter().min().unwrap();
    let max = lst.iter().max().unwrap();

    (*min, *max)
}

```

### Regex validate PIN code

```rust
Option
Some(_) =>
None =>
fn validate_pin(pin: &str) -> bool {
    let len = pin.len();
    if len == 4 || len == 6 {
        let mut ans = true;
        pin.chars().for_each(|x| match x.to_digit(10) {
            Some(_) => {}
            None => ans = false,
        });
        ans
    } else {
        false
    }
}
//
fn validate_pin(pin: &str) -> bool {
    pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}
fn validate_pin(pin: &str) -> bool {
    if ![4, 6].contains(&pin.len()) { return false; }
    pin.chars().all(|c| c.is_ascii_digit())
}
```

### Printer Errors

```rust
fn printer_error(s: &str) -> String {
    static ASCII_LOWER: [char; 13] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    ];
    let cnt = s
        .to_string()
        .chars()
        .into_iter()
        .filter(|x| !ASCII_LOWER.contains(x))
        .count()
        .to_string();

    let ans = cnt + "/" + &s.len().to_string();
    ans
}
//
fn printer_error(s: &str) -> String {
    // Your cude here
    format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}
fn printer_error(s: &str) -> String {
    let total = s.len();
    let bad = s.chars().filter(|&c| c < 'a' || c > 'm').count();
    format!("{}/{}", bad, total)
}
```

### Shortest Word

```rust
fn find_short(s: &str) -> u32 {
    s.to_string()
        .split(" ")
        .map(|x| x.len() as u32)
        .min()
        .unwrap()
}
//
fn find_short(s: &str) -> usize {
  s.split_whitespace().map(str::len).min().unwrap()
}
fn find_short(s: &str) -> u32 {
  s.split_whitespace()
   .map(|word| word.len())
   .min()
   .unwrap_or(0) as u32
}
```

### Growth of a Population

```rust
fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut cnt = 0;
    let mut total = p0 as f64;

    while total < p as f64 {
        total = total + (total * (percent / 100 as f64)) + aug as f64;
        total = total.floor();
        cnt += 1
    }
    cnt
}
```
