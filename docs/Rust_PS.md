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

0904

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

0907

### Number of People in the Bus

```rust
fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let mut come = 0;
    let mut out = 0;
    for peoples in bus_stops {
        let (push, pop) = peoples;
        come += push;
        out += pop;
    }
    come - out
}
//
fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0,|acc,x| acc + x.0 - x.1)
}
fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops
        .into_iter()
        .map(|n| n.0 - n.1)
        .sum()
}

```

0910

### Testing 1-2-3

```rust
fn number(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let mut str = String::from("");
            let idx = i + 1;
            let s = format!("{}: {}", idx, item);
            str.push_str(&s);
            str
        })
        .collect::<Vec<String>>()
}
//
fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|x| format!("{}: {}", x.0 + 1, x.1)).collect()
}
fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().zip(1..).map(|(x, i)| format!("{i}: {x}")).collect()
}
fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|(n, line)| format!("{}: {line}", n + 1)).collect()
}
```

### Complementary DNA

```rust
fn dna_strand(dna: &str) -> String {
    let mut ans = String::from("");
    dna.chars().for_each(|c| match c {
        'A' => ans.push_str("T"),
        'T' => ans.push_str("A"),
        'G' => ans.push_str("C"),
        _ => ans.push_str("G"),
    });
    ans
}
//
fn dna_strand(dna: &str) -> String {
  dna.chars().map(|c|
    match c {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        'C' => 'G',
        _ => c,
    })
    .collect()
}
use std::collections::HashMap;

fn dna_strand(dna: &str) -> String {
  let complements: HashMap<char, char> = [('A', 'T'), ('T', 'A'), ('C', 'G'), ('G', 'C')].iter().cloned().collect();
  dna.chars().map(|c| complements.get(&c).unwrap()).collect()
}
```

### Maximum Length Difference

into_iter, iter의 차이 https://sftblw.tistory.com/91

```rust
fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() == 0 || a2.len() == 0 {
        -1
    } else {
        let a_map1 = a1.into_iter().map(|x| x.len() as i32);
        let a_map2 = a_map1.clone();
        let b_map1 = a2.into_iter().map(|x| x.len() as i32);
        let b_map2 = b_map1.clone();
        //
        let x_max = a_map1.max().unwrap();
        let x_min = a_map2.min().unwrap();
        let y_max = b_map1.max().unwrap();
        let y_min = b_map2.min().unwrap();

        let a = (x_max - y_min).abs();
        let b = (y_max - x_min).abs();
        if a > b {
            a
        } else {
            b
        }
    }
}
//
use std::cmp::{max, min};
use std::usize::MAX;

pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }
    let (max1, min1) = a1
        .iter()
        .map(|&x| x.len())
        .fold((0, MAX), |ac, x| (max(ac.0, x), min(ac.1, x)));
    let (max2, min2) = a2
        .iter()
        .map(|&x| x.len())
        .fold((0, MAX), |ac, x| (max(ac.0, x), min(ac.1, x)));

    max(((max1 - min2) as i32).abs(), ((max2 - min1) as i32).abs())
}
fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() == 0 || a2.len() == 0 { return -1 }
    let a1_min = a1.iter().map(|s| s.len()).min().unwrap() as i32;
    let a1_max = a1.iter().map(|s| s.len()).max().unwrap() as i32;
    let a2_min = a2.iter().map(|s| s.len()).min().unwrap() as i32;
    let a2_max = a2.iter().map(|s| s.len()).max().unwrap() as i32;
    (a1_max - a2_min).max(a2_max - a1_min)
}
fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    // your code
    a1.iter().flat_map(|s1| a2.iter().map(|s2| (s1.len() as i32 - s2.len() as i32).abs()).collect::<Vec<_>>() ).max().unwrap_or(-1)
}
```

### Odd or Even?

```rust
fn odd_or_even(numbers: Vec<i32>) -> String {
    let sum: i32 = numbers.iter().sum();
    if sum % 2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}
//
fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() % 2 == 0 {
        true => "even".to_string(),
        false => "odd".to_string()
    }
}
fn odd_or_even(numbers: Vec<i32>) -> String {
    (if numbers.iter().sum::<i32>() % 2 == 0 {"even"} else {"odd"}).to_string()
}
```

### Check the exam

```rust

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let ans = arr_a
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            if arr_b[idx] == "" {
                0
            } else if &arr_b[idx] == val {
                4
            } else {
                -1
            }
        })
        .sum();
    if ans < 0 {
        0
    } else {
        ans
    }
}
//
fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a.iter().zip(arr_b.iter()).fold(0, |pts, ans| {
        match ans {
            (a, b) if a == b => pts + 4,
            (_, b) if b == &"" => pts,
            _ => pts - 1
        }
    }).max(0)
}
fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a
        .iter()
        .zip(arr_b)
        .map(|(&a, &b)| match b {
            "" => 0,
            _ if a == b => 4,
            _ => -1,
        })
        .sum::<i64>()
        .max(0)
}
```

### Highest and Lowest

```rust
fn high_and_low(numbers: &str) -> String {
    let vec = numbers.split(" ").map(|x| x.parse::<i32>().unwrap());
    let vec2 = vec.clone();
    let min = vec.min().unwrap();
    let max = vec2.max().unwrap();
    format!("{} {}", max, min)
}
//
fn high_and_low(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", answer.0, answer.1)
}
extern crate itertools;
use itertools::Itertools;

fn high_and_low(numbers: &str) -> String {
    let (min, max): (i32, i32) = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .minmax()
        .into_option()
        .unwrap();
    format!("{} {}", max, min)
}
fn high_and_low(numbers: &str) -> String {
  let as_ints: Vec<i32> = numbers.split(" ").map(|x| x.parse().unwrap()).collect();
  format!("{} {}", as_ints.iter().max().unwrap(), as_ints.iter().min().unwrap())
}
```

### Sum of Minimums!

```rust

fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers
        .map(|arr| arr.into_iter().min().unwrap())
        .iter()
        .sum()
}
//
fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|row| row.iter().min().unwrap()).sum()
}
fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().filter_map(|v| v.iter().min()).sum()
}
fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().flat_map(|x| x.iter().min()).sum()
}

```

0911

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```

###

```rust

```
