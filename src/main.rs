use console::Style;
use term_size::dimensions;

// use style like this:
// print!("{}", style("=").cyan());

fn main() {
    let def_length = 10;
    let def_lines = 5;
    // let term_width = get_terminal_width();
    let term_width = 100;

    let height = def_lines + 2;
    let width = def_length + 4;

    print_horizontal_line(term_width);

    // here should be read of file config
    // check_args_file() // or something like that

    // get cli config
    // let (l, h) = receive_config();
    // let length = l.unwrap_or(def_length);
    // let lines = h.unwrap_or(def_lines);
    // println!("l = {:?}, h = {:?}", length, lines);
    let length = def_length;
    let lines = def_lines;

    // // simulate interactive session like this
    // let (tier, item) = receive_new_item();
    // println!("tier {}, item {}", tier, item);

    // let cut_res = cut_string(&item, &length, &lines);
    // println!("result vec = {:?}", cut_res);

    // //  some setup for dynamic interaction
    // let tiers = ["S", "A", "B", "C", "D", "E"];

    // let mut items: Vec<Vec<String>> = Vec::new();

    // items.push(cut_res);
    // println!("{:?}", items);
    // items.push([["S"], ["item"], ["item"], ["item"]]);

    // // block print debug
    let tiers: [[&[&str]; 6]; 6] = [
        [&["S"], &["item"], &["item"], &["item"], &[], &[]],
        [&["A"], &["item"], &["item"], &["item"], &["item"], &[]],
        [
            &["B"],
            &["item"],
            &["item"],
            &["item"],
            &["item"],
            &["item"],
        ],
        [&["C"], &[], &[], &[], &[], &[]],
        [&["D"], &[], &[], &[], &[], &[]],
        [&["IDK"], &["item"], &["item"], &["item"], &[], &[]],
    ];

    for tier_index in 0..tiers.len() {
        print_blocks(&height, &width, &(tier_index as i32), tiers[tier_index]);
        print_horizontal_line(term_width);
    }
}

fn receive_new_item() -> (String, String) {
    let tier = std::env::args().nth(1).expect("Pass Tier and Item title");
    let item = std::env::args().nth(2).expect("Pass Item title");
    (tier.clone(), item.clone())
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
