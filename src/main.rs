mod first;
mod second;
mod model;
mod third;

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello, test");
}

#[test]
fn test_variable(){
    let name = "Dihanto";
    println!("Hello {}", name);
}

#[test]
fn test_mutable(){
    let mut name = "Angga";
    println!("Hello {}", name);
    name = "Budi";
    println!("Hello {}", name);
}

#[test]
fn shadowing(){
    let name = "Dihanto";
    println!("Hello {}", name);

    let name = "Angga";
    println!("Hello {}", name);
}

#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

}

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a/b;
    let e = a+b;
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}

#[test]
fn augmented_assigment(){
     let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean_operator(){
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type (){
    let char1 = 'a';
    let char2 = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple(){
    let mut data : (i32, f64, bool) = (10, 10.6, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    let (a,b ,c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    println!("{} {} {}", data.0, b, c);
}

fn unit(){
    println!("Hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array(){
    let mut array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    let c = array[2];
    println!("{} {} {}",a ,b ,c);

    array[0] = 10;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);

}

#[test]
fn two_dimensional_array(){
    let matrix: [[i32; 2]; 2] = [
        [1,2],
        [3,4]
    ];

    println!("{:?}", matrix);

    println!("{}", matrix[0][1]);
}

const  MAXIMUM : i32 = 100;

#[test]
fn constant(){
    const MINIMUM : i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("Kurniawan");
    println!("{} {}",a ,b);
}
fn function_b(){
    let a = 10;
    let b = String::from("Eko");
    println!("{} {}",a ,b);
}

#[test]
fn string(){
    let name = " Dihanto ";
    println!("{}", name);
    let trim = name.trim();
    println!("{}", trim)
}

#[test]
fn string_type(){
    let mut name = String::from("Dihanto");
    println!("{}", name);
    name.push_str(" Hansbreaker");
    println!("{}", name);

    let ujang = name.replace("Hansbreaker", "Ujang");
    println!("{}", name);
    println!("{}", ujang);
}

#[test]
fn ownership_rules(){
//     a tidak bisa diakses disini, belum dideklarasikan
     let a = 10; //a bisa diakses mulai disini
    {// b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai disini
        println!("{}", b);
    }// scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} //scopre a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy(){
    let a = 10;
    let mut  b = a;
    println!("{} {}", a, b);

    b = 23;
    println!("{} {}", a, b);
}


#[test]
fn ownership_movement(){
    let name1 = String::from("Dihanto");

    let name2 = name1; //ownership pindah ke name2

    println!("{}", name2);
}

#[test]
fn clone(){
    let name1 = String::from("Dihanto");
    let name2 = name1.clone();

    println!("{} {}", name1,name2);
}

#[test]
fn if_expression(){
    let value = 5;
    let result = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not bad"
    }else if value >= 3{
        "bad"
    } else {
        "Very bad"
    };
    println!("{}", result)
}

#[test]
fn loop_expression(){
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10{
            break;
        } else if counter % 2 == 0 {
            continue
        }
    println!("Counter  : {}", counter);
    }
}

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10{
            break counter *2;
        }
    };
    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop{
        let mut i =1;
        loop{
            if number > 10{
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i +=1;
            if i> 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10{
        if counter %2 == 0{
            println!("Counter : {}", counter)
        }
        counter += 1;
    }
}

#[test]
fn array_itteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;
    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_itteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for value in array{
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let array = ["A", "B", "C", "D", "E"];
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range{
        println!("{}", array[i])
    }
}

#[test]
fn range_inclusive() {
    let array = ["A", "B", "C", "D", "E"];
    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range{
        println!("{}", array[i])
    }
}

fn say_helllo(){
    println!("Hello")
}

#[test]
fn test_say_hello() {
    say_helllo();
    say_helllo();
    say_helllo();
}

fn say_goodbye(first_name: &str, last_name: &str){
    println!("Goodbye {} {}", first_name, last_name)
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Eko", "Khanedyy");
    say_goodbye("Eko", "Susilo");
}

fn factorial_loop (n: i32) -> i32{
    if n< 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);
}

fn print_text(value: String, times: u32){
    if times == 0{
        return;
    }else {
        println!("{}", value);
    }

    print_text(value, times-1)
}

#[test]
fn test_print_text() {
    print_text(String::from("Khanedy"), 12);
}

fn factorial_recursive(n: u32) -> u32{
    if n <= 1{
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number (number: i32){
    println!("number: {}", number);
}

fn hi(name: String){
    println!("Hi, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Dihanto");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String{
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Eko");
    let last_name = String::from("Khanedy");
    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value (value: &mut String ){
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Dika");
    change_value(&mut value);
    change_value(&mut value);
    change_value(&mut value);
    println!("{}", value);
}

#[test]
fn slice_reference() {
    let array = [1,2,3,4,5,6,7,8,9, 10];
    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("Eko Kurniawan Khanedy");
    let first_name : &str = &name[0..3];
    println!("{}", first_name);
    let last_name : &str = &name[14..];
    println!("{}", last_name);
}

struct Person {
    first_name : String,
    middle_name : String,
    last_name : String,
    age: u8,
}

#[test]
fn struct_person() {
    let first_name = String::from("Eko");
    let last_name = String::from("Khannedy");
    let person: Person = Person {
        first_name,
        middle_name: String::from("Kurniawan"),
        last_name,
        age: 22,
    };

    print_person(&person);
    let person2: Person = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);
    println!("{}", person.first_name);
}


fn print_person(person: &Person){
    println!("{}", person.first_name);
    println!("{}", person.age);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.32334234, 100.23023023);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;
#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing;
}

impl Person {
    fn say_hello(&self, name: &str){
        println!("Hello {}, my name is {}", name,  self.first_name);
    }
}

#[test]
fn test_method() {
    let person: Person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Kurniawan"),
        last_name: String::from("Khannedy"),
        age: 22,
    };

    person.say_hello("Budi");
}

impl GeoPoint{
    fn new(long: f64, lat: f64) -> GeoPoint{
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(-8.20132813, 78.123832);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level: Level = Level::Premium;
    let _level: Level = Level::Regular;
    let _level2: Level = Level::Platinum;
    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

enum Payment{
    CreditCart(String),
    BankTransfer(String, String),
    Ewallet(String, String),
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCart(String::from("234234234"));
    _payment1.pay(50000);
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("2342342"));
    _payment2.pay(500120);
    let _payment3: Payment = Payment::Ewallet(String::from("Gopay"), String::from("2342342"));
    _payment3.pay(2349234);
}

impl Payment {
    fn pay(&self, amount: u32){
        match self{
            Payment::CreditCart(number) => {
                println!("paying with credit card {} amount {}", number, amount)
            }
            Payment::BankTransfer(bank, number) => {
                println!("paying with bank transfer {} {} amount {}", bank, number, amount)
            }
            Payment::Ewallet(wallet, number) => {
                println!("Paying with ewallet {} {} amount {}", wallet, number, amount)
            }
        }
    }
}

#[test]
fn test_match_value() {
    let name = "Budi";
    match name {
        "Eko" => {
            println!("Hello Eko");
        }
        "Budi" => {
            println!("Hello Budi");
        }
        other => {
            println!("Hello {}", other)
        }
    }

    match name {
        "Eko"|"Budi" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other)
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 100;

    match value{
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not bad")
        }
        0..=24 => {
            println!("Bad")
        }
        other => {
            println!("Invalid value {}", other)
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint(1.0, 11.0);
    match point {
        GeoPoint(long, 0.0) => {println!("long: {}", long)}
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat)
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat)
        }
    }

    let person: Person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Kurniawan"),
        last_name: String::from("Khannedy"),
        age: 22,
    };

    match person {
        Person{first_name, last_name, ..} => {
            println!("{} {}", first_name, last_name)
        }
    }
}

#[test]
fn test_ignoring() {
    let point =  GeoPoint(0.0, 11.0);

    match  point {
        GeoPoint(long, _) => {
            println!("long: {}", long)
        }
    }
}

#[test]
fn test_ignoring_range_patterns() {
    let value = 100;

    match value{
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not bad")
        }
        0..=24 => {
            println!("Bad")
        }
        _ => {
            println!("Invalid value")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;
    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };

    println!("{}", result)
}

type Age = u8;
type IdentityNumber = String;

struct Customer{
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_type_alias() {
    let customer = Customer{
        id: String::from("21342423432"),
        name: String::from("Eko"),
        age: 20,
    };
    println!("{} {} {}", customer.id, customer.name, customer.age)
}
#[test]
fn test_module() {
    let user = model::User{
        first_name: String::from("Eko"),
        last_name: String::from("Khanedy"),
        username: String::from("EkoKhanedy"),
        email: String::from("eko@gmail.com"),
        age: 12,
    };

    user.say_hello("Budi")
}

use first::say_hello;
use second::say_hello as say_hello_second;
#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    first::second::third::say_hello()
}

trait CanSayHello{
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye{

    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person{

    fn say_hello(&self) -> String {
        format!("Hello,my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodBye for Person{
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}",self.first_name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello){
    println!("{}", value.say_hello())
}
fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)){
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}
#[test]
fn test_trait() {
    let person: Person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Kurniawan"),
        last_name: String::from("Khannedy"),
        age: 22,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let result = person.say_hello_to("Budi");
    println!("{}", result);
    let result = person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Umar"));

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Sule");
}

struct SimplePerson {
    name: String
}

impl CanSayGoodBye for SimplePerson{
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person (name: String) -> impl CanSayGoodBye{
    SimplePerson{name}
}
#[test]
fn test_return_trait() {
    let person = create_person(String::from("Ekooo"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Usman"));
}

trait CanSay : CanSayHello + CanSayGoodBye{
    fn say(&self){
        println!("{}", self.say_hello());
        println!("{}", self.good_bye())

    }
}

struct Point<T = i32>{
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> {
        x: 1, y:2,
    };
    let float = Point::<f64>{
        x: 1.1, y:2.0,
    };

    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y)
}

enum Value<T>{
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);
    match value{
        Value::NONE => {
            println!("None");
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

struct Hi<T> where T: CanSayGoodBye{
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson>{
        value: SimplePerson{
            name: String::from("Sahrul")
        }
    };
    println!("{}", hi.value.name)

}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
 if value1 < value2 {
     value1
 } else {
     value2
 }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10,20);
    println!("{}", result)
}

impl<T> Point<T>{
    fn get_x(&self) -> &T{
        &self.x
    }
    fn get_y(&self) -> &T{
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y: 20};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());


}

trait GetValue<T> where T: PartialOrd{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd{
    fn get_value(&self) -> &T{
        &self.x
    }
}

struct Apple{
    quantity: i32,
}

use core::ops::Add;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

impl Add for Apple {
    type Output = Apple;
    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity: self.quantity + rhs.quantity,
        }
    }
}



#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity: 6};
    let apple2 = Apple{quantity: 7};

    let apple3 = apple1 + apple2;

    println!("{}", apple3.quantity);
}

fn double(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i *2),
    }
}

#[test]
fn test_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result)
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_compare() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("Apple1 == Apple2 : {}", apple1 == apple2);
    println!("Apple1 < Apple2 : {}", apple1 < apple2);
    println!("Apple1 > Apple2 : {}", apple1 > apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Hari ini adalah hari minggu");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("minggu", "senin"));
    println!("{}", s.contains("minggu"));
    println!("{}", s.starts_with("Hari"));
    println!("{}", s.ends_with("minggu"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter};
impl Debug for Category{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
#[test]
fn test_formatting() {
    let category = Category {
        name: String::from("Gadget"),
        id: String::from("GADGET")
    };

    println!("{:?}", category)
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32{
        value1 + value2
    };

    let result = sum(1,2);
    println!("Result : {}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String){
    let result = filter(value);
    println!("Result: {}", result);
}

#[test]
fn test_closure_as_parameter() {
    let filter = |value: String | -> String{
        value.to_uppercase()
    };
    print_with_filter(String::from("Eko"), filter)
}

fn to_uppercase(value: String) -> String{
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("Eko"), to_uppercase)
}

struct Counter {
    counter: i32,
}

impl Counter{
    fn increment(&mut self){
        self.counter += 1;
        println!("Increment")
    }
}

#[test]
fn  test_counter() {
    let mut counter = Counter{counter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter : {}", counter.counter)

}
#[test]
fn test_closure_scopre() {
    let mut counter = 0;
    let mut increment = ||
        {
            counter += 1;
            println!("Increment");
        };
    increment();
    increment();
    increment();

    println!("Counter: {}", counter)
}

#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Eko"));
    names.push(String::from("Kurniawan"));
    names.push(String::from("Khanedy"));

    for name in &names{
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_vec_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Eko"));
    names.push_back(String::from("Kurniawan"));
    names.push_front(String::from("Khannedy"));

    for name in &names{
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Eko"));
    names.push_back(String::from("Kurniawan"));
    names.push_front(String::from("Khannedy"));

    for name in &names{
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("name"), String::from("Eko"));
    map.insert(String::from("age"), String::from("25"));

    let name = map.get("name");
    let age = map.get("age");

    println!("{}", name.unwrap());
    println!("{}", age.unwrap())
}

#[test]
fn test_btree_map() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    map.insert(String::from("name"), String::from("Eko"));
    map.insert(String::from("age"), String::from("25"));
    map.insert(String::from("country"), String::from("Indonesia"));


    for entry in map{
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_hash_set() {
    let mut set : HashSet<String> = HashSet::new();
    set.insert(String::from("Eko"));
    set.insert(String::from("Eko"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Khannedy"));
    set.insert(String::from("Khannedy"));

    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_btree_set() {
    let mut set : BTreeSet<String> = BTreeSet::new();
    set.insert(String::from("Eko"));
    set.insert(String::from("Eko"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Kurniawan"));
    set.insert(String::from("Khannedy"));
    set.insert(String::from("Khannedy"));

    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_iterator() {
    let array: [i32; 5] = [1,2,3,4,5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next(){
        println!("{}", value);
    };

    for value in iterator {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("Vector: {:?}", vector);
    let sum: i32 = vector.iter().sum();
    println!("Sum : {}", sum);

    let counter = vector.iter().count();
    println!("{}", counter);

    let doubled : Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>){
    match host {
        Some(host) => {
            println!("Connecting to database {}", host)
        }
        None => {
            panic!("No database host provided")
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("Localhost")));
    // connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String>{
    match host{
        Some(host) => Ok(host),
        None => Err("No cache host provided".to_string())
    }
}

#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Success connect to host : {}", host)
        }
        Err(error) => {
            println!("Error with message : {}", error)
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String>{
    match host{
        Some(host) => Ok(host),
        None => Err("No email host provided".to_string())
    }
}

fn connect_application(host: Option<String>) -> Result<String, String>{
    connect_cache(host.clone())?;
    connect_email(host.clone())?;

    Ok("Connected to application".to_string())
}

#[test]
fn test_application_error() {
    let result = connect_application(None);
    match result {
        Ok(host) => {println!("success connect with message : {}", host)}
        Err(err) => {println!("error with message : {}", err)}
    }
}