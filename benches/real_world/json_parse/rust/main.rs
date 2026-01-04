// JSON parsing benchmark
// Measures: String processing, recursive descent parsing

fn is_ws(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn skip_ws(s: &[u8], pos: usize) -> usize {
    let mut p = pos;
    while p < s.len() && is_ws(s[p]) {
        p += 1;
    }
    p
}

fn find_close(s: &[u8], mut pos: usize, open: u8, close: u8) -> usize {
    let mut depth = 0i32;
    while pos < s.len() {
        let c = s[pos];
        if c == close && depth == 0 {
            return pos;
        } else if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
        }
        pos += 1;
    }
    pos
}

fn count_array_elements(s: &[u8], pos: usize) -> i64 {
    let p = skip_ws(s, pos);
    if p >= s.len() || s[p] != b'[' {
        return 0;
    }

    let p2 = skip_ws(s, p + 1);
    if p2 >= s.len() || s[p2] == b']' {
        return 0;
    }

    let mut count = 1i64;
    let mut depth = 0i32;
    let mut i = p2;

    while i < s.len() {
        let c = s[i];
        if c == b']' && depth == 0 {
            break;
        } else if c == b'[' || c == b'{' {
            depth += 1;
        } else if c == b']' || c == b'}' {
            depth -= 1;
        } else if c == b',' && depth == 0 {
            count += 1;
        }
        i += 1;
    }

    count
}

fn validate_json(s: &[u8], pos: usize) -> bool {
    let p = skip_ws(s, pos);
    if p >= s.len() {
        return false;
    }

    let c = s[p];
    match c {
        b'{' => {
            let close = find_close(s, p + 1, b'{', b'}');
            close < s.len()
        }
        b'[' => {
            let close = find_close(s, p + 1, b'[', b']');
            close < s.len()
        }
        b'"' => {
            // Find closing quote
            let mut i = p + 1;
            while i < s.len() {
                if s[i] == b'"' {
                    return true;
                }
                if s[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            false
        }
        b't' | b'f' | b'n' => true,  // true/false/null
        _ if is_digit(c) || c == b'-' => true,  // number
        _ => false,
    }
}

fn run_benchmark(iterations: i64) -> i64 {
    let json = b"[1,2,3,4,5,6,7,8,9,10]";
    let mut valid_count = 0i64;

    for _ in 0..iterations {
        let is_valid = if validate_json(json, 0) { 1 } else { 0 };
        let element_count = count_array_elements(json, 0);
        valid_count += is_valid + element_count;
    }

    valid_count
}

fn main() {
    let result = run_benchmark(10000);
    println!("{}", result);
}
