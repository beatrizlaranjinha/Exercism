pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => 
                {
                open.push(c);
                }
            '}' | ')' | ']' =>
                {
                    let open2 = open.pop();
                    if let None= open2 {
                        return false;
                    }
                    match open2 {
                        None => return false,
                        Some(open2) => {
                            match (open2, c) {
                                ('(' ,')') => {}
                                ('{','}') => {}
                                ('[',']') => {}
                                _ => return false,
                            }
                        }
                    }
               }
            _ => {}
        }
    }
    open.is_empty()
}

