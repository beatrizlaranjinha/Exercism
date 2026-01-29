pub fn series(digits: &str, len: usize) -> Vec<String> {
    let bytes = digits.as_bytes(); 

    if len == 0 || len > bytes.len()  {
        return vec![];
    }

        let mut aux = Vec::new();

    for w in bytes.windows(len){
        let slice = std::str::from_utf8(w).unwrap();
        aux.push(slice.to_string());
    }
    aux
}
