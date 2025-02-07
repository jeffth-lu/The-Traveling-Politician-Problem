// function to convert String to static str.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// function to create cypher.csv heading
pub fn title(vec_len: usize) -> Vec<String> {
    let mut header: Vec<String> = Vec::new();

    for i in 0..vec_len {
        match i {
            0 => header.push("KEY".to_owned()), // First header column
            a if a == (vec_len - 1) => header.push("DISTANCE".to_owned()), // Last header column
            _ => {
                // Configure header for each state in vector
                let mut a: String = "STATE_".to_owned();
                let b: String = i.to_string();
                let b: &'static str = string_to_static_str(b);
                a.push_str(b);
                header.push(a);
            }
        }
    }
    header // return header
}

// function constructs the rows of cypher.csv
pub fn vec_row(row_num: usize, distance: f64, mut vec_row: Vec<&str>) -> Vec<&str> {
    let mut vec: Vec<&str> = Vec::new();

    let rownum = format!("{:?}", row_num);
    let dist = format!("{:.1}", distance);
    let rownum: &'static str = string_to_static_str(rownum);
    let dist: &'static str = string_to_static_str(dist);

    vec.push(rownum); // Key: row numbers
    vec.append(&mut vec_row); // States
    vec.push(dist); // Distance
    vec // return vec
}
