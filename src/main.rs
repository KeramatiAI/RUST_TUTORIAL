mod data_types;
use data_types::DataFields;

fn main() {
    // Method 1
    let data_fields = DataFields {
        first_name: "davoud".to_string(),
        last_name: "keramati".to_string(),
        nickname: "daker".to_string(),
        age: 25, // مقدار age در data_fields برابر 25 است
        national_code: "00103050654".to_string(),
        weight: 76.5, // مقدار weight در data_fields برابر 76.5 است
        height: 179, // مقدار height در data_fields برابر 179 است
        married: false,
        first_letter_of_name: 'D',
        is_alive: false, // مقدار is_alive در data_fields برابر false است
    };

    // برای چاپ struct از :? استفاده کنید چون Debug را اضافه کردیم
    println!("data_fields: {:?}", data_fields);


    // Method 2

    let name = data_fields.first_name; // استفاده از clone برای String
    let family = data_fields.last_name;
    let nickname = data_fields.nickname;
    let first_letter_of_name = data_fields.first_letter_of_name;
    let age = data_fields.age;
    let national_code = data_fields.national_code;
    let weight = data_fields.weight;
    let height = data_fields.height;
    let married = data_fields.married;
    let is_alive = data_fields.is_alive;

    println!(
        "information => name:{},
    family:{},
    nickname:{},
    first letter of name:{},
    age:{},
    national code:{},
    weight:{},
    height:{},
    married:{},
    is_alive:{}"
        , name, family, nickname, first_letter_of_name, age, national_code, weight, height, married, is_alive
    );
}
