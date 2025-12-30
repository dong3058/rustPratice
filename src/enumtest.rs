pub fn enum_test(){

    let male=Gender::MALE;

    match male{
        Gender::MALE=>println!("man"),
        Gender::FEMALE=>println!("female")
    }

    //참고로 option,result도 enum에 속한다.
    //option에는 some(T),none이 존재한다.--> 즉 자바의 optional비슷한것으로 vec,map에서 특정값에대해서 get접근시
    //option중에서 none, 값이 존재하면 some으로 돌려준다.
    let num_str="010-5805-3058";
    let mut vec=Vec::new();
    num_str.chars()//chars는 문자열을 1개의 char(4byte)로 순회시킴.
    .filter_map(|x| x.to_digit(10))
    .for_each(|x| vec.push(x) );
    println!("{:?}",vec);


    //option이 값이 없는 경우를 위한것이면 result는 에러가 발생한 경우를 위한것
    match divmod(1,0){
        Ok(n)=>println!("{:?}",n),
        Err(n)=>println!("{}",n),
    }
}

enum Gender{
    MALE,FEMALE
}

fn divmod(x:i32,y:i32)->Result<(i32,i32),String>{
    if y==0{
        return Err("0으로 나누기 불가함".to_owned());
    }
    return Ok((x/y,x%y));
}
/*
enum Gender {
    Male {name:String, is_military:bool},
    Female {name:String}
}

fn get_customer(id:i32) -> Gender {    
    if id % 2 == 0 {
        return Gender::Male{name:"Jeff".to_owned(), is_military:true};
    }
    return Gender::Female {name:"Alice".to_owned()};
}

fn main() {
    let gender = get_customer(10);
    match gender {
        Gender::Male{name:n, is_military:b} =>{
            println!("name={}, is_military={}",n,b);
        },
        Gender::Female{name:n} => {
            println!("name={}",n);
        },
    }
}

다음과 ㄱ같이 enum을 구조체스럽게 사용할수도있다.

*/ 