// 라이프 타임의 문제

/*
fn main(){
    let r;  // (1)

    {
        let x = 5;  // (2)
        r = &x;  // (3)
    }

    println!("r: {}", r);  // (4)
}
    해당 코드는 에러가 발생하는대 그 이유는 x는 블록에서 이미끝나버린걸 밖의 r에서 참조하려고하기떄문.
    이런 문제를 라이프 타임 문제라고한다-->즉 생명주기 가 긴 변수가 생명주기가 짧은 변수를 참조하려고할때 발생할수잇는 문제다.
*/
/*
fn main(){
    let x = String::from("123");     // (1)
    let y = String::from("45678");
    let s = longest(x.as_str(), y.as_str());  // (2)
    println!("longest is : {}",s);
}

fn longest(s1:&str, s2:&str) -> &str {    // error: 라이프타임 파라미터가 필요
    if s1.len() > s2.len() { s1 }     
    else { s2 }
} 
다음코드도 라이프 타임에러가 발생하는대 이는 if,else에 의해서 어떤 변수가 반환되는지 모르기에
어떤 라이프 타임으로 결정해야될지를 모르기떄문.-->즉 둘다 변수를 참조 값으로 빌려주고 있는 상황이라
longest라는 함수의 return값의 라이프 타임을 x로잡을지,y로잡을지 알수가 없다 이말이다.


Rust 컴파일러는 모든 참조값에 대해서 라이프타임 변수를 할당해서, 그 라이프타임이 언제 종료되는 지를 체크한다.
*/

/*

fn longest<'a>(s1:&'a str, s2:&'a str) -> &'a str {    
    if s1.len() > s2.len() { s1 }     
    else { s2 }
} 
에서 제네릭인 'a 가바로 라이프타임 문제를 해결하는애인대 애는 쉽게 생각해서
두변수 s1,s2중 짧은 애의 라이프 타임을 컴파일시점에 부여하겠다는 의미이다.

fn main(){
    let x = String::from("123");
    let s; 
    {
        let y = String::from("45678");
        s = longest(x.as_str(), y.as_str());
    }
    println!("longest is : {}",s);  //error
}
해당 함수에서 longest의 라이프타임은 s1,s2두개의 변수 즉 y와 x의 변수중 라이프타임이짧은 y로 라이프 타임을가진다.
-->실제로 y가 리턴되는게 아니라 컴파일 시점에서 변수 y가 x보다 짧은 라이프 타임을 가지므로 longest로 같은 라이프
타임을 가지게 설정해서 컴파일을 한다는것.
*/

/*
함수에서는 리턴값이 참조 타입의 경우, 무조건 입력되는 파라미터의 라이프타임을 가지고 표기해야 한다.
fn main(){
    let x = String::from("123");
    let s;
    {
        let y = String::from("45678");  // (1)
        s = firt_str(x.as_ref(), y.as_ref());  // (2)
    }
    println!("The first string is : {}",s);
}

fn firt_str<'a>(s1:&'a str, s2:&str) -> &'a str { // (3) 
    println!("s2={}",s2);  
    s1
} 
즉 이런꼴인대 중간의 {}안에 firt_str메서드가 들어가고 리턴값이 &str이라는 참조값이므로 입력되는
파라미터의 라이프 타임을 가져야된다.
여기서 dangular pointer문제를 피하기위해서 밖의 x라는 긴 라이프 타임을 가지는 변수 자리 s1의 `a라는 라이프
타임을 가지도록 팔라미터 앞에 붙인것.
fn main(){
    let x = String::from("123");
    let s;
    {
        let y = String::from("45678");
        s = firt_str(x.as_ref(), y.as_ref());
    }
    println!("The first string is : {}",s);
}

fn firt_str<'a>(s1:&'a str, s2:&'a str) -> &'a str {  
    println!("s2={}",s2);  
    s1
} 
애는 에러인대 라이프타임이 2개의 파라미터에 대해서 모두 작성되었으므로 짧은 주기를 가지기떄문.
만약 리턴값이 참조값인대 파라미터가 1개면 굳이 적을필요없다.->1개의 파라미터와 라이프타임이 같게 계산하기떄문.
*/

/*
구조체에서는 그 구성원으로 참조자를 가지고 있는 경우 무조건 해당 구조체에 대해 라이프타임을 표기해줘야 한다.
struct Point<'a, 'b>{
    x:&'a i32,
    y:&'b i32,
}

fn main(){
    let x1 = 10;
    let p;
    {
        let y1 = 20;
        p = Point {x:&x1, y:&y1};
    }
    println!("point = {:?}",p);
}
    다음코드는 에러가 발생하는대 구조체는 해당 구성원보다 수명이길면안된다.
    즉 y1은 진작에 끝낫는대 p라는애는 수명이 더길기에 발생하는 dangular pointer 문제이다.

    이는곧 구조체의 수명을 구조체 구성원중 가장 수명이짧은 애들보다 길게잡을수 없다 라고하는것이다.
    러스트는 구조체 선언시 모든 참조자 형태의 구성원에 대해서 라이프타임을 반드시 표기하게한다.

    //[코드 2]
#[derive(Debug)]
struct Point<'a>{
    x:&'a i32,
    y:&i32,
}

fn main(){
    let x1 = 10;
    let p;
    {
        let y1 = 20;
        p = Point {x:&x1, y:&y1};
    }
    println!("point = {:?}",p);
}

    즉 다음코드는 y에 라이프타임 변수가 없어서 발생하는 문제.

    struct Borrowed<'a>(&'a i32);

fn main(){
    let x = 10;
    let b = Borrowed(&x);

    println!("borrowed: {}",b.0);
}
    튜플 구조체도 딱히 다를게없다.

구조체에 라이프타임이 표시되었다면 구조체를 impl할때에도 라이프 타임을 넣어야된다.
struct Person<'a>{
    name: &'a str,
    age: i32,
}

impl<'a> Person<'a> { }

이런거.

구조체에 대한 impl에서 메서드를 구현할 때는 일반 함수에서의 라이프타임 표기 방법과 유사하다.

1.리턴 타입이 참조자 형태가 아니면 라이프타임 표기 필요 없다.
2.리턴 타입이 self에 있는 구성원에 대한 참조자이면, 라이프타임 표기를 생략할 수 있다. 리턴값에 대한 라이프타임이 self의 라이프타임 매개변수와 같게 된다.
3.리턴 타입이 참조자 형태이고 2번 케이스가 아니면 라이프타임 표기를 해야한다.

impl<'a> Person<'a> {
    fn get_age(&self) -> i32 { self.age }     
}-->애가 1번 케이스인대 단순i32반환이라 표기 안해도됨.


   fn get_name(&self) -> &str { self.name }  

    fn print_alias(&self, alias:&str ) -> &str {
        println!("alias:{}",alias);
        self.name
    }
    애는 리턴하는게 참조자는 맞는대 어차피 self의 구성원을 리턴하는거라 생략해도됨.


        fn longest_name<'b>(&'b self, s:&'b str ) -> &'b str {
        if self.name.len() > s.len() {self.name}
        else {s}
    }
    애는 리턴하는 str이 self의것 인지 혹은 외부변수 s인지 알수가 없기에--> 라이프타임을 결정해줘야된다

    struct Person<'a>{
    name: &'a str,
    age: i32,
}

impl<'a> Person<'a> {
    fn get_age(&self) -> i32 { self.age }  

    fn get_name(&self) -> &str { self.name }  
    fn print_alias(&self, alias:&str ) -> &str {
        println!("alias:{}",alias);
        self.name
    }

    fn longest_name<'b>(&'b self, s:&'b str ) -> &'b str {
        if self.name.len() > s.len() {self.name}
        else {s}
    }    
}

fn main(){
    let p = Person {
        name: "Jeff", age: 10,
    };
    println!("age: {}", p.get_age());
    println!("name: {}", p.get_name());
    println!("name: {}", p.print_alias("park"));
    println!("longest: {}", p.longest_name("abcdefg"));
}
    즉이런꼴이다.

*/