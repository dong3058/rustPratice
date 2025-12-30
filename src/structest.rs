pub fn struct_practice(){

    let mut student=Student{
        age:12,name:"hello".to_owned()
    };
    println!("{:?}",student);

    student.age=13;
    student.name="world".to_owned();

    println!("{:?}",student);


    let mut tuple=Tuples(12,12,12);

    println!("{:?}",tuple);
    tuple.0=13;
    tuple.1=13;
    tuple.2=13;

    println!("{:?}",tuple);


    let mut p1=Point::new(1,1);
    let p2=Point::new(2,2);
    let ans=p1.distance(&p2);
    println!("{:?}",ans);
    p1.x=2;
    println!("{:?}",p1);

    let ans2=distance_ver2(&p1,&p2);
    println!("{:?}",ans2)
}

//구조체인대 java의 class로 생각하면될듯.
#[derive(Debug)]
struct Student{
    age:i32,
    name:String
}
//tuple 구조체 프로퍼티 접근시에는 tuple.1,tuple.2 이렇게 접근.
#[derive(Debug)]
struct Tuples(i32,i32,i32);

#[derive(Debug)]
struct Point{
    x:i32,
    y:i32
}

impl Point{
    fn new(x:i32,y:i32)->Point{
        Point{x:x,y:y}
    }
    //애는 메서드인대 함수와 메서드의 차이는 메서드는 &self라는게 들어간다.
    fn distance(&self,p:&Point)->(i32,i32){
        (self.x-p.x,self.y-p.y)
    }
}

fn distance_ver2(p1:&Point,p2:&Point)->(i32,i32){
    // "."연산자 사용시에 역참조 표시를 해줄필요없다. 알아서 처리해줌 ㅇㅇ;
    p1.distance(p2)

}

//구조체 자체는 스택에 저장된다.
//물론 구조체의 프로퍼티들은 그 타입에따라서 스택 혹은 힙에 저장될것이다.
//그러나 구조체는 copy trait이 없으므로 변수에 넘겨주면 소유권 자체가 이전된다.


//json에 비어있는 requestbody를 보낼시 빈 구조체를 정의해서 보내주면된다.