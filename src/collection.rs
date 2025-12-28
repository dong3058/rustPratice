use std::collections::HashMap;

pub fn collections(){


    //자바의 arraylist임. 참고로 rust는 따로 stack이 없고 vec이 stack의 역할을 수행한다.
    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    //컬랙션 계열-->vec or map or set or array들은 전체 출력하려면 :? 로 해줘야된다.
    println!("{:?}",vec);
    vec.pop();
    println!("{:?}",vec);
    vec.extend([3,4,5,6]);
    println!("{:?}",vec);
    //그냥 인덱스 접근은 없는 인덱스에대해서 panic 즉 에러 터트리는대
    //get은 자바의 optional처럼 안전하게 none을 반환.
    match vec.get(11){
        Some(n)=>println!("{n}"),
        None=>println!("none"),
    }

    vec.reverse();
    println!("{:?}",vec);
    vec.sort();
    println!("{:?}",vec);


    //자바의 hash map에해당.
    let mut map=HashMap::new();
    map.insert("a",1);
    map.insert("b",2);
    println!("{:?}",map);

    //map 도 get할시에 option을 던져준다.
    match map.get("c"){
        Some(n)=>println!("{n}"),
        None=>println!("None"),
    }

    let test=&map;
    //value에대한 참조를 주는게 맞는대 println은 자동으로 역참조해서 some으로 준다. 없다면 none임.
    println!("{:?}",test.get("a"));


}