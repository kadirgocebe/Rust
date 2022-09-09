fn main() { //main fonksiyonu

    print_numbers_to(10); //içeride bir func cağırdı
 
/*
    if is_even(21){             //basit if else ama bunu diğer func içine taşıdık
        println!("It is even");
    }
    else {
        println!("It is odd");}*/
}





fn print_numbers_to(num: u32){ //func içine u32 aldı void bir func

    for n in 1..num {  //basit for dongusu
          
        //println!("{}", n); //bu içeride basıyordu 
        
        if is_even(n){          //if elsi burda koyduk
                println!("{} is even!", n);
            }
            else {
                println!("{} is odd !",n);
            }
     }
}





fn is_even(num: u32) -> bool{ //bu da için u32 aldı ama ok işareti değer dönecek demek değer de bool olacak

    return num % 2 == 0;  // bool bir işlem verdin döndü
}



