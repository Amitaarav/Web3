#[derive(Debug, Copy, Clone, PartialEq)]
enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

impl Inventory {
        fn giveaway (&self, user_preferences: Option<ShirtColor>) -> ShirtColor{
            user_preferences.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor{
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }

            if num_red > num_blue {
                ShirtColor :: Red
            } else {
                ShirtColor::Blue
            }
        }
}


pub fn closures(){
    let outer_var = 42;

    // fn function(i: i32) -> i32 {
    //     i + outer_var
    // }

    let closure_annotated = |i: i32| -> i32 {
        i + outer_var
    };

    let closure_inferred = |i|      i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));

    println!("closure_inferred: {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning one: {}", one());


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    let store = Inventory {
        shirts : vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The use with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // 

    let add_one_v2 = |x: i32, y: i32| -> i32  {
        x + y
    };

    let add_one_v3 = |x, y|  {
        x + y
    };

    println!("add_one_v2 : {}", add_one_v2(5, 3));
    println!("add_one_v3 : {}", add_one_v3(5, 3));

}