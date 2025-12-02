pub fn task1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let id_ranges: Vec<&str> = input.trim().split(',').collect();
    let mut result= 0;

    for id_range in id_ranges {
        let (start_str,end_str) = id_range.split_once("-").ok_or("range cannot be split")?;
        let start: i64 = start_str
            .parse()
            .map_err(|e| format!("could not parse '{}': {}", start_str, e))?;
        let end: i64 = end_str
            .parse()
            .map_err(|e| format!("could not parse '{}': {}", end_str, e))?;

        for id in start..=end {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue
            } 
            let (first,second) = id_str.split_at(id_str.len()/2);

            if first == second {
                result += id;
            }
        }
    }

    Ok(result)
}

pub fn task2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let id_ranges: Vec<&str> = input.trim().split(',').collect();
    // let id_ranges = vec!["998-1112"];
    let mut result= 0;

    for id_range in id_ranges {
        let (start_str,end_str) = id_range.split_once("-").ok_or("range cannot be split")?;
        let start: i64 = start_str
            .parse()
            .map_err(|e| format!("could not parse '{}': {}", start_str, e))?;
        let end: i64 = end_str
            .parse()
            .map_err(|e| format!("could not parse '{}': {}", end_str, e))?;

        for id in start..=end {
            let id_str = id.to_string();
            for i in 1..id_str.len() {
                let substr: String = id_str.chars().take(i).collect();
                if id_str == substr.repeat(id_str.len()/i) {
                    // println!("id found: {}", id_str);
                    result += id;
                    break;
                }
            }
        }
    }
    Ok(result)
}