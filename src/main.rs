use std::io;
use std::io::Write;
use std::thread::sleep;

#[test]
fn solve() -> Result<(), ()> {
    let p1 = "03706332";
    let p2 = "20231185";
    let p3 = "Ok`xV0sgQt4s";

    // inverse p1
    let mut inv_p1 = String::new();
    for c in p1.chars() {
        if let Some(digit) = c.to_digit(16) {
            let transformed = format!("{:x}", digit ^ 0xD);
            inv_p1.push_str(&transformed);
        } else {
            return Err(());
        }
    }

    inv_p1 = inv_p1.to_uppercase();

    // inverse p2
    let inv_p2 = format!("{:08}", u32::from_str_radix(p2, 10).unwrap() ^ 0x23);

    // inverse p3
    let mut inv_p3 = String::new();
    for c in p3.chars() {
        if c.is_ascii() {
            inv_p3.push(((c as u8).wrapping_add(0x1)) as char);
        } else {
            return Err(());
        }
    }

    let flag = format!("YPCTF{{{}_{}_{}}}", inv_p1, inv_p2, inv_p3);
    println!("{}", flag);
    Ok(())
}

fn main() {
    println!("Enter your flag:");
    let mut flag = String::new();
    io::stdin().read_line(&mut flag).expect("Failed to read line");
    let flag = flag.trim();

    // YPCTF{DEADBEEF_20231218_PlayW1thRu5t}
    if flag.starts_with("YPCTF{") && flag.ends_with("}") {
        let content = &flag[6..flag.len()-1];
        let parts: Vec<&str> = content.split('_').collect();
        if parts.len() == 3 {
            if let (Ok(hex), Ok(date), ascii) = (hex_to_string(parts[0]), date_xor(parts[1]), ascii_sub(parts[2])) {
                print!("Nice! Let's see what we got: ");
                let ascii = ascii.as_ref().map(|s| s.as_str()).unwrap_or("Invalid");

                for _ in 0..3 {
                    print!(".");
                    io::stdout().flush().unwrap();
                    sleep(std::time::Duration::from_millis(1000));
                }
                println!();
                if hex == "03706332" && date == "20231185" && ascii == "Ok`xV0sgQt4s" {
                    println!("You got it!");
                } else {
                    println!("Oops, something is wrong...");
                }
            } else {
                println!("No!");
            }
        } else {
            println!("You?");
        }
    } else {
        println!("...what?");
    }
}

fn hex_to_string(hex_str: &str) -> Result<String, ()> {
    let mut result = String::new();
    for c in hex_str.chars() {
        if c.is_lowercase() {
            return Err(());
        }
        if let Some(digit) = c.to_digit(16) {
            let transformed = format!("{:x}", digit ^ 0xD);
            result.push_str(&transformed);
        } else {
            return Err(());
        }
    }
    Ok(result)
}

fn date_xor(date_str: &str) -> Result<String, ()> {
    if date_str.len() == 8 && date_str.chars().all(char::is_numeric) {
        let date = u32::from_str_radix(date_str, 10).unwrap();
        Ok(format!("{:08}", date ^ 0x23))
    } else {
        Err(())
    }
}

fn ascii_sub(ascii_str: &str) -> Result<String, ()> {
    let mut result = String::new();
    for c in ascii_str.chars() {
        if c.is_ascii() {
            result.push(((c as u8).wrapping_sub(0x1)) as char);
        } else {
            return Err(());
        }
    }
    Ok(result)
}

