#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    }
    let mut ans = 0;
    let mut a = vec![false; n];
    for i in 0..n {
        if i < k {
            ans += score(t[i], r, s, p);
            continue;
        }
        if t[i] != t[i - k] {
            ans += score(t[i], r, s, p);
        } else {
            // t[i] == t[i - k]
            if a[i - k] == true {
                ans += score(t[i], r, s, p);
            } else {
                a[i] = true;
            }
        }
    }
    println!("{}", ans);
}
fn score(a: char, r: usize, s: usize, p: usize) -> usize {
    match a {
        'r' => p,
        's' => r,
        'p' => s,
        _ => 0,
    }
}
