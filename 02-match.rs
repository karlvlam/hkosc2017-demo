fn main(){
    let numbers = [0,1,2,3,4,5,18,22,30,39,40];
    for number in &numbers { // 0 to 31
        let result = match *number {
            1 => "咪一囉！", // expression
            2|3|5|7 => "質數啲Friend!",
            18|22 => "傳說中的十八、廿二",
            30...39 => "三張嘢……",
            _ => "其他，係咁多", // uncomment and see...
        };

        println!("{} -> {}", number, result);
    }

}
