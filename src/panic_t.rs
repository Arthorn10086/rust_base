fn speak(s:&str){
    if s=="sb" { panic!("不准人身攻击！"); }
    println!("{}",s)
}

pub fn main(){
    speak("hello");
    speak("sb");
    speak("world");
}