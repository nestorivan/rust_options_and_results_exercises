// fn main() {
//     let username = get_username(1);

//     // if username matches the pattern (Some(name))
//     // the content of the some variant will be mapped to the name variable
//     if let Some(name) = username {
//         println!("{:?}", name);
//     }
// }

// fn get_username(user_id: i32) -> Option<String> {
//     let db_result = String::from("testman");
//     if user_id == 1 {
//         Some(db_result)
//     } else {
//         None
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let y: Option<&Point> = Some(&Point { x: 100, y: 200 });

//     match y {
//         Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
//         _ => println!("no match"),
//     }
//     y; // Fix without deleting this line.
// }

// fn last_element(nums: &[i32]) -> Option<i32> {
//     if nums.len() > 0 {
//         Some(nums[nums.len() - 1])
//     } else {
//         None
//     }
// }

// fn main() {
//     let my_nums = [1, 1, 2, 3, 5, 8, 13];
//     match last_element(&my_nums) {
//         Some(ele) => println!("Last element: {ele}"),
//         None => println!("Empty array"),
//     }
// }

struct User {
    id: u32,
    name: String,
}

fn get_user_name(id: u32) -> Option<String> {
    let database = [
        User {
            id: 1,
            name: String::from("Alice"),
        },
        User {
            id: 2,
            name: String::from("Bob"),
        },
        User {
            id: 3,
            name: String::from("Cindy"),
        },
    ];
    for user in database {
        if user.id == id {
            return Some(user.name);
        }
    }
    None
}

fn main() {
    let user_id = 3;
    if let Some(name) = get_user_name(user_id) {
        println!("User's name: {name}");
    } else {
        println!("User not found");
    }
}
