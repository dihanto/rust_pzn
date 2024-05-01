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