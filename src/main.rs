fn main() {

    //Boolean
    let _first_bool = true;//Thue undescore is used to avoid warning of unused variable
    let _second_bool = false;

    //Integer
    let _weekdays: i8 = 7;
    let _number_of_users: i64 = 128000;
    let _number_of_tokens: u64 = 10000; // unsigned integer
    let _just_a_number  = 0;// will default to i32

    //Floating point
    let _pi: f32 = 3.14;

    //Character
    let _my_char: char = 'A';

    //String
    let _message = "Hello, test!";//string slice
    let _my_string = String::from("Hello, test!");//String

    //Array
    let _days_of_week: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let _first_element = _days_of_week[0];
    let _last_element = _days_of_week[_days_of_week.len() - 1];

    //Slice
    let slice = &_days_of_week[0..3];
    let _first_element = slice[0];

    //tuple
    let person = ("James", "Bond", 7);
    let _first_name = person.0;
    let _age = person.2;

    //Unit type
    let _unit_type = ();

    //variables
    let mut num = 5;
    num = 6;


}
