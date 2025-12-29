pub fn trait_test(){
    let truck=Truck::make_truck("truck".to_owned());
    truck.show_car_name();


    let p1=Point::create_point(1,1);
    let p2=Point::create_point(2,2);
    let p3=p1.add(&p2);
    println!("{:?}",p3);

    let h=Human{
        name:"tom".to_owned()
    };
    let animal=Animal{
        name:"dog".to_owned()
    };

    use_run(&h);
    use_run(&animal);

}


//trait은 자바의iterface 비슷한것.
//자바의 class를 struct로 구현이되나 struct간의 상속은 존재하지않음. 이러한 공통된 부문을 trait으로 구현함.


struct Truck{
    name:String
}


//이건 truck이라는 클래스에서 쓸 메서드를 정의하는거고
impl Truck{
    fn make_truck(name:String)->Self{
        Truck{name:name}
    }
}

//type이 뭐냐면 해당 trait을 구현한것에 대한 메타 데이터를 의미한다. 즉 구현체의 타입을 의미.
//아래의 Self는 정해진 키워드로써 해당 trait을 구현한 것의 데이터 타입을 말한다. 여기서는 Truck을 의미한다.
//위의 구조체 구현에서 Truck이 아닌 return 타입을 Self로 준것은 구조체 그자신의 타입을 의미한다.
//self와 Self는 엄연히 다른대 앞의 것은 그자신 그자체를 의미하고 뒤의것은 메타데이터 즉 타입을 말한다.
//즉 메서드 내부에 파리미터를 넘길때 &self, x:&Self 에서 이렇게 넘기는대 여기서부터 차이가 보인다 이말.

trait car{
    type output;
    fn show_car_name(&self);
}

//애는 car라는 interface를 truck이라는 클래스 전용으로써 구현하는것. 
//이렇게하ㅕㄴ 추후에 trcuk을 car로 casting해서 다루수있따.
//즉 truck이란게 car라는 인터페이스를 구현한것이다 라고보면된다.
impl car for Truck{
    type output=Self;

    //Self::output==> 자기자신의 것중에서 output이라는 애를 찾아라. 즉 ::는 ~안에있는 즉 Self안에있는 output을 의미한다.
    fn show_car_name(&self){
        println!("this is {}",self.name)
    }

}
//type vs 제네릭
//둘의 차이는 구현체 즉 구조체가 몇개의trait을 가질수있는가를 의미한다.
//trait add<T>일경우 구조체는 해당 trait을 무수히 많이 가질수있따-->T가 매번다르면되니까
//그러나 만약 trait안에 type으로 지정시 구조체는 1개의 trait만을 가질수있다.

#[derive(Debug)]
struct Point{
    x:i32,
    y:i32
}
impl Point{
    fn create_point(x:i32,y:i32)->Self{
        Point{x:x,y:y}
    }
}
trait Add{
    type output;

    fn add(&self,val:&Self)->Self::output;
}
impl Add for Point{
    type output=Self;

    fn add(&self,val:&Self)->Self::output{
        Self{
            x:self.x+val.x,
            y:self.y+val.y
        }
    }
}


struct Human{
    name:String
}
struct Animal{
    name:String
}

trait Run{

    fn run(&self);
}
impl Run for Human{

    fn run(&self){
        println!("human-{} is running",self.name)
    }
}
impl Run for Animal{
    fn run(&self){
        println!("animal-{} is running",self.name)
    }
}
//다형성에 관한 내용인대 rust는 정적 바인딩을 지원한다. 
//만약 return 부문에 사용되면 컴파일 시점에서 어떤 타입이 리턴될지 알수있어야된다.
fn use_run(x: &impl Run){
    x.run();
}

//아래코드는 실패한다. 컴파일 시점에서 어떤 값이 return되는지 결정할수없다 왜?--> if문으로 인해서 분기가되기때문.
//즉 둘다 run을 구현하긴하였으나 확정적으로 어떤 타입이 return되는지 모르기떄문이다.
/*fn failcase(x:bool)->impl Run{
    if(x){
        Human{
            name:"test"
        }
    }
    else{
        Animal{
            name:"test"
        }
    }
}*/
//이코드느 에러가 발생하지않음-->즉 동적 바인딩.
fn failcase(x:bool)->Box<dyn Run>{
    if(x){
        Box::new(Human{
            name:"test".to_owned()
        })
    }
    else{
        Box::new(Animal{
            name:"test".to_owned()
        })
    }
}