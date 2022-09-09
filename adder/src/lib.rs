pub fn add_two(a: i32 ) -> i32 { //burda bir func yazdın test için
    a + 2
}
pub fn greeting(name: &str) -> String{
    format!("Hello {}!" , name)
}

#[cfg(test)]        
mod tests {     //test modulünde çalıştırıcaksın
    use super::*;   //genel olarak kullansın diye süper yaptın

    #[test]         // bu test edilecek demek
    fn it_adds_two() {  //fonksiyonu yazdın
        
        assert_eq!(4, add_two(2));  // basit check islemi ilk terim kontrol terimi ikincisi checker
        assert_ne!(3, add_two(4)); // buda not equal mı diye bakıyor
    }
  
    #[test]         // bu test edilecek demek
    fn greeting_contains_name() {  //fonksiyonu yazdın
        let result : String = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain Carol value was '{}'",
            result
        ); 
    }

    #[test]         // bu test edilecek demek
    fn it_works() -> Result<(), String> { 
        if 2 + 2 == 4{
            Ok(())
        } else {
            Err(String::from( "two plus two does not equal four!" ))
        }

    }
  
}
