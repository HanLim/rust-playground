use std::collections::HashMap;
fn main() {
    let nums = vec![1, 2, 5, 1, 3, 9];
    println!("median: {}", median(&nums));
    println!("mode: {}", mode(&nums));

    println!("apple: {}", pig_latin(&"apple".to_string()));
    println!("first: {}", pig_latin(&"first".to_string()));

    let mut dept: HashMap<String, Vec<String>> = HashMap::new();
    add_to_dept(&"Add Sally to Engineering".to_string(), &mut dept);
    add_to_dept(&"Add Amir to Sales".to_string(), &mut dept);
    add_to_dept(&"Add Akir to Sales".to_string(), &mut dept);

    dbg!(dept.get("Engineering"));
    dbg!(dept.get("Sales"));
}

fn median(nums: &Vec<i32>) -> f64 {
    let mut nums = nums.clone();
    nums.sort_unstable();

    match nums.len() % 2 {
        0 => {
            let mid = nums.len() / 2;
            (nums[mid - 1] + nums[mid]) as f64 / 2.0
        }
        _ => nums[(nums.len() as f64 / 2.0).floor() as usize] as f64,
    }
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for num in nums {
        let val = hm.entry(num).or_insert(0);
        *val += 1;
    }
    **hm.iter().max_by_key(|entry| entry.1).unwrap().0
}

fn pig_latin(word: &String) -> String {
    let first = word.chars().next().unwrap().to_lowercase().next().unwrap();
    match "aeiou".find(first) {
        Some(_) => format!("{}-hay", word),
        None => {
            let mut chars = word.chars();
            chars.next();
            format!("{}-{}ay", chars.as_str(), first)
        }
    }
}

fn add_to_dept(instruction: &String, dept: &mut HashMap<String, Vec<String>>) {
    let instructions: Vec<&str> = instruction.split_ascii_whitespace().collect();
    let name = instructions[1].to_string();
    let department = instructions[3].to_string();

    dept.entry(department.clone()).and_modify(|v| {
        match v.binary_search(&name) {
            Ok(_) => {},
            Err(index) => v.insert(index, name.clone())
        }
    }).or_insert(vec![name.clone()]);
}
