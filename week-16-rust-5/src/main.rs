use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Address{
    city_name: String,
    pin_code: String
}
#[derive(Serialize, Deserialize, Debug)]
struct SignupResponse{
    message: String,
    address: Address
}
fn main() {
    let address = Address{
        city_name: String::from("Prayagraj"),
        pin_code: String::from("212109")
    };

    let s = SignupResponse{
        message: String::from("You are not signed in"),
        address
    }; 

    let json_str = serde_json::to_string(&s).unwrap();

    let s2:Result<SignupResponse> = serde_json::from_str(&json_str);
    print!("{}", json_str);
    println!("\n");
    print!("{:?}", s2.unwrap());
}
