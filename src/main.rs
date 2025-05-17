use console::Style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use term_size::dimensions;

#[derive(Default, Serialize, Deserialize)]
struct TierList {
    data: HashMap<String, Vec<String>>,
}

const DEFAULT_TIERS: &[&str] = &["S", "A", "B", "C", "D"];
const TIER_COLORS: &[u8] = &[196, 208, 214, 226, 118, 247, 122, 087, 033, 099, 247];

impl TierList {
    // fn new() -> Self {
    //     TierList {
    //         data: HashMap::new(),
    //     }
    // }

    fn init_with_tiers(tiers: &Vec<String>) -> Self {
        let mut tier_list = TierList::default();
        for tier in tiers {
            tier_list.data.insert(tier.to_string(), Vec::new());
        }
        tier_list
    }

    fn insert_item(&mut self, tier: &str, item: String) {
        if let Some(tier_data) = self.data.get_mut(tier) {
            tier_data.push(item);
        }
        // self.data
        //     .entry(tier.to_string())
        //     .or_insert_with(Vec::new)
        //     .push(item);
    }

    // fn get_all_tiers(&self) -> Vec<&String> {
    //     self.data.keys().collect()
    // }

    // fn get_tier(&self, tier: &str) -> Option<(String, &Vec<String>)> {
    //     match self.data.get(tier) {
    //         Some(items) => Some((tier.to_string(), items)),
    //         None => None,
    //     }
    // }

    fn get_tier_as_array(&self, tier: &str) -> Option<Vec<String>> {
        match self.data.get(tier) {
            Some(values) => {
                let mut tier_items_array = vec![tier.to_string()];
                tier_items_array.extend_from_slice(values);
                Some(tier_items_array)
            }
            None => None,
        }
    }

    fn from_json_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let tier_list: TierList = serde_json::from_reader(reader)?;
        Ok(tier_list)
    }

    fn to_json_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(file_path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}

fn main() {
    // max length of one text line
    // and max lines per tier block
    let def_length = 9;
    let def_lines = 5;

    let term_width = get_terminal_width();

    // borders around text:
    // +2 for width so the tiers aren't too thin
    // +0 for height because with 5 lines and
    // arbitrary space above/below it is enough
    let width = def_length + 2;
    let height = def_lines;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 && args[1] == "--json" {
        // JSON file read

        let file_path = &args[2];
        let tier_list = TierList::from_json_file(file_path).unwrap();
        let tiers_len = tier_list.data.len();

        let mut tiers: Vec<String> = tier_list.data.keys().cloned().collect();
        tiers.sort_by(|a, b| {
            let a_value = match a.chars().next().unwrap_or('Z') {
                'S' => 0,
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                c if c.is_ascii_uppercase() => c as u8 - b'E' + 5,
                _ => 31, // for any other tier
            };
            let b_value = match b.chars().next().unwrap_or('Z') {
                'S' => 0,
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                c if c.is_ascii_uppercase() => c as u8 - b'E' + 5,
                _ => 31, // for any other tier
            };
            a_value.cmp(&b_value)
        });

        // duplicate of tier list print (single iteration for JSON input)
        print_horizontal_line(term_width);

        for tier_index in 0..tiers_len {
            if let Some(items) = tier_list.get_tier_as_array(&tiers[tier_index]) {
                print_tier(
                    &height,
                    &width,
                    &(tier_index as i32),
                    &items,
                    &def_length,
                    &def_lines,
                );

                print_horizontal_line(term_width);
            }
        }
    } else {
        // CLI interactive input and print

        println!("Enter tiers, split by ; (empty to use S,A,B,C,D)");
        let mut tiers_str = String::new();
        std::io::stdin()
            .read_line(&mut tiers_str)
            .expect("Failed to read tiers");
        tiers_str = tiers_str.trim().to_string();

        let tiers = if tiers_str.is_empty() {
            DEFAULT_TIERS.iter().map(|s| s.to_string()).collect()
        } else {
            parse_tiers(&tiers_str)
        };

        let tiers_len = tiers.len();
        let mut tier_list = TierList::init_with_tiers(&tiers);

        loop {
            // (first print current tiers, then prompt)
            print_horizontal_line(term_width);

            // HashMap doesnt keep order, so we'll have to keep it ourselves
            for tier_index in 0..tiers_len {
                if let Some(items) = tier_list.get_tier_as_array(&tiers[tier_index]) {
                    print_tier(
                        &height,
                        &width,
                        &(tier_index as i32),
                        &items,
                        &def_length,
                        &def_lines,
                    );

                    print_horizontal_line(term_width);
                }
            }

            println!("\nEnter tier and item separated by = ('q' to exit):");
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read item");

            // quit option by passing
            // q
            if input.trim() == "q" {
                break;
            }

            // write current tier list to JSON file by passing
            // --to-json /path/to/some_file.json
            if input.starts_with("--to-json") {
                let json_file_path = &input[9..].trim();
                match tier_list.to_json_file(json_file_path) {
                    Ok(_) => {
                        println!("\x1b[32mExport to {} successful\x1b[0m\n", json_file_path);
                    }
                    Err(e) => {
                        println!("\x1b[31mExport failed: {}\x1b[0m\n", e);
                    }
                }
                continue;
            }

            // split input like
            // tier=item
            let input_split: Vec<&str> = input.trim().split("=").collect();

            if input_split.len() == 2 {
                tier_list.insert_item(input_split[0], input_split[1].to_string());
            }
        }
    };
}

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

fn parse_tiers(tiers_str: &str) -> Vec<String> {
    tiers_str.split(";").map(|s| s.to_string()).collect()
}

fn cut_string(input_string: &str, max_length: &i32, lines: &i32) -> Vec<String> {
    // here we put the algo to cut item title for printing

    // println!(
    //     "input= {} len = {} lines = {}",
    //     input_string, max_length, lines
    // );

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
    // if tier index > len, reuse last element (gray color)
    let tier_index = if *tier >= TIER_COLORS.len() as i32 {
        TIER_COLORS.len() - 1
    } else {
        *tier as usize
    };

    let colored_item = Style::new()
        .color256(TIER_COLORS[tier_index] as u8)
        .apply_to("@");
    print!("{}", colored_item);
}

fn print_tier(height: &i32, width: &i32, tier: &i32, items: &[String], length: &i32, lines: &i32) {
    // prints single tier without top horizontal line
    // accepts an array (tier name + items)

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
    // end print top row

    println!();
    // print the lines between top - bottom
    for line in 0..*height {
        print!("||");

        // print tier name (first item in items)
        print_tier_color(tier);
        let tier_strings = cut_string(&items[0], length, lines);

        let lines_above = (*height - tier_strings.len() as i32) / 2;
        let lines_until_below = lines_above + tier_strings.len() as i32;

        if line - lines_above >= 0 && line < lines_until_below {
            let current_str = &tier_strings[(line - lines_above) as usize];

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
        // end print tier name

        print_tier_color(tier);
        print!("||");

        // print items
        for index in 1..items.len() {
            let item_strings = cut_string(&items[index], length, lines);

            let lines_above = (*height - item_strings.len() as i32) / 2;
            let lines_until_below = lines_above + item_strings.len() as i32;

            if line - lines_above >= 0 && line < lines_until_below {
                let current_str = &item_strings[(line - lines_above) as usize];

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
    // end print bottom row
    println!();
}
