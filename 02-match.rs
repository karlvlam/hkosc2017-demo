fn main(){
    let numbers = [0,1,2,3,4,5,18,22,30,39,40,-41];
    for number in &numbers { 
        let result = match *number {
            1 => "咪一囉！", // expression
            2|3|5|7 => "質數啲Friend!",
            18|22 => "傳說中的十八、廿二",
            30...39 => "三張嘢……",
            // comment below and see...
            _ => 
                if number < &0 {
                    "-這是負數-" 
                }else{
                    "+這是正數+" 
                }
        };

        println!("{} -> {}", number, result);
    }

}
