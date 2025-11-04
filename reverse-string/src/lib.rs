pub fn reverse(input: &str) -> String {

    let mut chars: Vec<char> = input.chars().collect();
    // let iterator = chars.iter();
    let mut reversed = Vec::<char>::new();

    while let Some(c) = chars.pop(){
        reversed.push(c);
    }

    reversed.iter().collect()
}
