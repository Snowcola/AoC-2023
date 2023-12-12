use std::fs;

pub fn solve() {
    let file_path = "input/2023/day2.txt";
    let res: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            let game_id =  match line.split(":").collect::<Vec<&str>>().first() {
                Some(x) => x.split(" ").collect::<Vec<&str>>().first(),
                _ => ""
            }
            
            ;
         
            println!("{:?}", game_id);
            match line.split(":") {
                _ => ()
            }
            line.to_string()

        })
        .collect();
        
}