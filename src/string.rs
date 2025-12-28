pub fn str_test(){

    //러스트이 문자열 다루는 방식은 &str,string 2가지가 존재
   
    //&str은 str의 주소값 즉 참조값을 의미.
    //아래 처럼 선언시 변수는 일반적으로 &str즉 hello world의 참조값을 지니게된다.
    // 이는 보통의 프로그래밍 언어에서 문자열이 원시타입이면서 실질적으론 레퍼런스 타입의 형태를 가지는것과 비슷한
    //것으로 생각된다.
    let s="hello world";

    //개별적인 인덱스 접근은 안되나 범위 단위로 슬라이싱 접근은된다.
    //이는, utf-8 방식으로 인코딩 되어 있어, 
    //하나의 문자의 바이트 단위가 1이 아닌 1~4 바이트이기에, 단일 인덱스로의 접근을 아예 막은 것이라 한다. 
    //대신 슬라이스 범위로의 접근은 가능하다.
    println!("{}",s.len());
    println!("{}",&s[0..2]);

    let s2="대한민국";
    println!("{}",&s2[0..3]);//대 출력됨.

    //string은 vector라고 생각해도 무방. 단 애도 idx접근이 안됨.이건 &str과 같은 성질이다.
    //&s를 string으로 바꾸는 과정 to_string이라는 메서드도있는대 to_owned가 좀더 가볍다고한다.
    let mut string1="hi".to_owned();
    string1.replace_range(1..2, "o");
    println!("{:?}",string1);


    //&str타입은 해당 변수의 value는 힙메모리,스택이 아닌 다른 읽기전용 메모리공간에 적재되고
    //&str타입의 변수는 이 value의 borrwos를 한것이다.
    //반면 string타입은 힙메모리에 저장 밎 소유권을 가지게된다.
    //아래에서 num4는 &str취급이다 즉 phone_num이라는 string의 슬라이싱 8idx~끝까지의 값들을 가져와서 참조(borrows)하므로
    //&str에 해당된다.
    let person = "Jeff";
    let phone_num = String::from("010-123-4567");
    let num4 = &phone_num[8..]; 
}   