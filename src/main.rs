fn main(){
    let number_list = vec![10,20,30,40,50];
    let char_list = vec!['a','b','c'];

   

  println!("{}", get_largest(number_list));
  println!("{}", get_largest(char_list));
}

fn get_largest<T: PartialOrd + Copy> (number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list{
        if number > largest{
            largest = number;
        }
    }
    largest
}