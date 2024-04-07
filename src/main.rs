use console::Style;
use std::collections::HashMap;
use std::io;
use term_size::dimensions;

// use style like this:
// print!("{}", style("=").cyan());

#[derive(Default)]
struct TierList {
    data: HashMap<String, Vec<String>>,
}

impl TierList {
    fn new() -> Self {
        TierList {
            data: HashMap::new(),
        }
    }

    fn insert_item(&mut self, tier: &str, item: String) {
        self.data
            .entry(tier.to_string())
            .or_insert_with(Vec::new)
            .push(item);
    }

    fn get_all_tiers(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    fn get_tier(&self, tier: &str) -> Option<(String, &Vec<String>)> {
        match self.data.get(tier) {
            Some(items) => Some((tier.to_string(), items)),
            None => None,
        }
    }
}

fn main() {
    let def_length = 10;
    let def_lines = 5;
    let term_width = get_terminal_width();

    let height = def_lines + 2;
    let width = def_length + 4;

    print_horizontal_line(term_width);

    let mut tier_list = TierList::new();

    loop {
        // mock data print (will be replaced by actual tier list
        // from the previous stage)
        // (first print current tiers, then prompt)
        println!("Data so far:");
        for tier in tier_list.get_all_tiers() {
            if let Some((tier_name, items)) = tier_list.get_tier(tier) {
                println!("{}: {:?}", tier_name, items);
            }
        }

        println!("Enter tier and item separated by ; ('q' to exit):");
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_split: Vec<&str> = input.trim().split(";").collect();

        if input.trim() == "q" {
            break;
        }
        if input_split.len() == 2 {
            tier_list.insert_item(input_split[0], input_split[1].to_string());
        }
    }

    // here should be read of file config
    // check_args_file() // or something like that

    // // get cli config
    // let (l, h) = receive_config();
    // let length = l.unwrap_or(def_length);
    // let lines = h.unwrap_or(def_lines);
    // println!("l = {:?}, h = {:?}", length, lines);
    // let length = def_length;
    // let lines = def_lines;

    // let cut_res = cut_string(&item, &length, &lines);
    // println!("result vec = {:?}", cut_res);

    // // sort of real code
    // for tier_index in 0..tiers.len() {
    //     print_blocks(&height, &width, &(tier_index as i32), tiers[tier_index]);
    //     print_horizontal_line(term_width);
    // }
}

// fn receive_tier_item() -> Option<(String, String)> {
//     println!("Enter tier and item separated by '; ':");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//     let input_split: Vec<&str> = input.trim().split("; ").collect();
//     if input_split.len() == 2 {
//         Some((input_split[0].to_string(), input_split[1].to_string()))
//     } else {
//         None
//     }
// }

// fn receive_config() -> (Option<i32>, Option<i32>){
//     // receive init args to set custom item line length
//     // and/or block height in lines (see, docs)

//     let args: Vec<String> = std::env::args().collect();

//     let mut length: Option<i32> = None;
//     let mut height: Option<i32> = None;

//     for pair in args[1..].chunks(2) {
//         if pair[0] == "length" {
//             length = pair[1].parse().ok();
//         } else if pair[0] == "height" {
//             height = pair[1].parse().ok();
//         }
//     }

//     (length, height)
// }

fn get_terminal_width() -> i32 {
    if let Some((width, _)) = dimensions() {
        // cap width at 150
        if width < 150 {
            width as i32
        } else {
            150
        }
    } else {
        // Default width if unable to get the terminal size
        80
    }
}

fn print_horizontal_line(term_w: i32) {
    for _ in 0..term_w {
        print!("=");
    }
    println!();
}

fn cut_string(input_string: &str, max_length: &i32, lines: &i32) -> Vec<String> {
    // here we put the algo to cut item title for printing

    println!(
        "input= {} len = {} lines = {}",
        input_string, max_length, lines
    );

    let words: Vec<&str> = input_string.split_whitespace().collect();

    let mut result = Vec::new();
    let mut current_line_length = 0;
    let mut current_line = 0;
    let mut current_str = String::new();

    for s in words.iter() {
        if current_line >= *lines as usize {
            break;
        }
        if current_line_length + s.chars().count() > *max_length as usize {
            if current_str.chars().count() > *max_length as usize {
                // trim words too long for a single line
                result.push(current_str.split_at(*max_length as usize).0.to_string());
            } else {
                result.push(current_str.trim().to_string());
            }
            current_str.clear();
            current_line += 1;
        }

        current_str.push_str(*s);
        current_str.push_str(" ");
        current_line_length = current_str.chars().count();
    }

    // put the rest
    if !current_str.is_empty() && current_line <= *lines as usize {
        result.push(current_str.trim().to_string());
    }

    result
}

fn print_tier_color(tier: &i32) {
    let colored_item = match tier {
        0 => Style::new().color256(196).apply_to("@"),
        1 => Style::new().color256(202).apply_to("@"),
        2 => Style::new().color256(220).apply_to("@"),
        3 => Style::new().color256(226).apply_to("@"),
        4 => Style::new().color256(118).apply_to("@"),
        5 => Style::new().color256(247).apply_to("@"),
        _ => Style::new().apply_to("@"),
    };
    print!("{}", colored_item);
}

fn print_blocks(height: &i32, width: &i32, tier: &i32, items: [&[&str]; 6]) {
    // print top row, which is arbitrary space
    print!("||");
    for _ in 0..*width {
        print_tier_color(tier);
    }
    print!("||");

    for i in 1..items.len() {
        if !items[i].is_empty() {
            for _ in 0..*width {
                print!(" ");
            }
            print!("|");
        }
    }
    // for _ in 0..items.len() -1 {
    //     for _ in 0..*width {
    //         print!(" ");
    //     }
    //     print!("|");
    // }
    // end print top row

    println!();
    // print the lines between top - bottom
    for line in 0..*height {
        print!("||");

        // print tier (first item in items)
        print_tier_color(tier);
        // print!("{}", style("@").red());

        let lines_above = (*height - items[0].len() as i32) / 2;
        let lines_until_below = lines_above + items[0].len() as i32;

        if line - lines_above >= 0 && line < lines_until_below {
            let current_str = items[0][(line - lines_above) as usize];

            let spaces_left = (*width - 2 - current_str.chars().count() as i32) / 2;
            let spaces_right = *width - (spaces_left + current_str.chars().count() as i32) - 2;
            for _ in 0..spaces_left {
                print!(" ");
            }

            print!("{}", current_str);

            for _ in 0..spaces_right {
                print!(" ");
            }
        } else {
            for _ in 0..*width - 2 {
                print!(" ");
            }
        }
        // end print tier line

        print_tier_color(tier);
        print!("||");

        // print items
        for index in 1..items.len() {
            if items[index].len() == 0 {
                break;
            }
            let lines_above = (*height - items[index].len() as i32) / 2;
            let lines_until_below = lines_above + items[index].len() as i32;

            if line - lines_above >= 0 && line < lines_until_below {
                let current_str = items[index][(line - lines_above) as usize];

                let spaces_left = (*width - current_str.chars().count() as i32) / 2;
                let spaces_right = *width - (spaces_left + current_str.chars().count() as i32);
                for _ in 0..spaces_left {
                    print!(" ");
                }

                print!("{}", current_str);

                for _ in 0..spaces_right {
                    print!(" ");
                }
            } else {
                for _ in 0..*width {
                    print!(" ");
                }
            }

            print!("|");
        }

        println!();
    }

    // print bottom row, which is arbitrary space
    print!("||");
    for _ in 0..*width {
        print_tier_color(tier);
    }
    print!("||");

    for i in 1..items.len() {
        if !items[i].is_empty() {
            for _ in 0..*width {
                print!(" ");
            }
            print!("|");
        }
    }
    // for _ in 0..items.len() -1 {
    //     for _ in 0..*width {
    //         print!(" ");
    //     }
    //     print!("|");
    // }
    // end print bottom row
    println!();
}
