use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all="camelCase")]
//#[serde(deny_unknown_fields)]
// unknown fileds will cause error
// default is to ignore unknow field
// e.g breed
struct Dog{
    name: String,
    year_born: i32,
    //#[serde(rename = "dog_owner")]
    // rename json field dog_owner to owner when deserialize
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all="camelCase")]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    serialize();
    deserialize();
}

fn serialize(){
    let owner01 = DogOwner{
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };

    let dog01 = Dog{
        name: String::from("Buddy"),
        year_born: 2018,
        owner: owner01,
    };

    println!("Seialize Dog: ");
    let dog_ser = to_string(&dog01);
    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

fn deserialize() {
    let json_string: &str= r#"
    {"name":"Buddy","year_born":2018,
    "owner":{"first_name":"John","last_name":"Doe"},
    "breed": "Collie"
    }
    "#;

    let dog_deser = from_str::<Dog>(json_string);
    
    println!("Deseialize Dog: ");
    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }

}