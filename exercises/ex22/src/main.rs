#[derive(Debug)]
enum ValidationError {
    TooShort { min: usize, got: usize },
    TooLong { max: usize, got: usize },
    InvalidCharacter(char),
    EmptyInput,
    OutOfRange { min: i32, max: i32, got: i32 },
}

impl ValidationError {
    fn validate_username(username: &str) -> Result<&str, ValidationError> {
        if username == "" {
            return Err(ValidationError::EmptyInput);
        } else {
            let length = username.chars().count();
            if length < 3 {
                return Err(ValidationError::TooShort {
                    min: 3,
                    got: length,
                });
            } else if length > 20 {
                return Err(ValidationError::TooLong {
                    max: 20,
                    got: length,
                });
            } else {
                for c in username.chars() {
                    if !c.is_alphanumeric() && c != '_' {
                        return Err(ValidationError::InvalidCharacter(c));
                    }
                }
            }
        }

        Ok(username)
    }

    fn validate_age(age_str: &str) -> Result<u32, ValidationError> {
        for c in age_str.chars() {
            if !c.is_numeric() {
                return Err(ValidationError::InvalidCharacter(c));
            }
        }

        match age_str.parse::<u32>() {
            Ok(n) => {
                if n < 13 || n > 120 {
                    return Err(ValidationError::OutOfRange {
                        min: 13,
                        max: 120,
                        got: n as i32,
                    });
                }
                Ok(n)
            }
            Err(_) => Err(ValidationError::EmptyInput),
        }
    }

    fn validate_password(password: &str) -> Result<&str, ValidationError> {
        if password == "" {
            return Err(ValidationError::EmptyInput);
        }
        let length = password.chars().count();
        if length < 8 {
            return Err(ValidationError::TooShort {
                min: 8,
                got: length,
            });
        } else if length > 50 {
            return Err(ValidationError::TooLong {
                max: 50,
                got: length,
            });
        }

        Ok(password)
    }
}

fn main() {
    println!("=== Valiation System ===");

    let usernames = ["Cove", "ab", "", "a very long username here", "(invalid)"];
    let ages = ["25", "10", "abc", ""];
    let passwords = ["secure123", "short"];

    // Usernames
    println!("");
    for username in usernames {
        println!(
            "Username {:<13} : {}",
            format!("\"{}\"", username),
            match ValidationError::validate_username(username) {
                Ok(_) => format!("✅ Valid"),
                Err(error) => format!("❌ {:?}", error),
            }
        );
    }

    // Ages
    println!("");
    for age in ages {
        println!(
            "Age {:<7} : {}",
            format!("\"{}\"", age),
            match ValidationError::validate_age(age) {
                Ok(age) => format!("✅ Valid - {}", age),
                Err(error) => format!("❌ {:?}", error),
            }
        );
    }

    // Passwords
    println!("");
    for password in passwords {
        println!(
            "Password {:<12} : {}",
            format!("\"{}\"", password),
            match ValidationError::validate_password(password) {
                Ok(_) => format!("✅ Valid"),
                Err(error) => format!("❌ {:?}", error),
            }
        );
    }
}
