fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut palindrome = true;
    for i in 0..chars.len()/2 {
        if chars[i] != chars[chars.len()-1-i] {
            palindrome = false;
            break;
        }
    }
    return palindrome;
}

fn main() {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let cur = i * j;
            if is_palindrome(cur.to_string()) && cur > max {
                max = cur;
            }
        }
    }
    println!("{}", max);
}