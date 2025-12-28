pub fn iteraion_test(){
    //반복자는 엄밀한 의미에서 collection과는 다르다
    //즉 collection을 하나하나 꺼내볼수있게 만든게 반복자라는 개념.
    //자바의 stream을 적용하는것처럼 이용도 가능함.


    /*
    iter() : 불변 대여 반복자가 적용되는 변수의 요소들의 레퍼런스를 가져옴. 소유권을 가져오는 형태가아니다.
    //iter를 하는값이 어떤 원본의 참조값이면 원본요소의 참조값을 가져온다.-->그러니까 알아서 똑똑하게 직관적으로 작용한다이말. 
    into_iter() : 원본값들의 iteration으로 변환후 원본은 사라짐.-->소유권 이전 이라기보단 원본의 iteration을 만들고 원본 삭제라고 보는게 맞다.
    iter_mut(): 가변 대여 컬렉션의 값을 수정해야할 때 사용한다. 레퍼런스로 받은 다음에 수정하는 것이다. 소유권이 넘어가지는 않는다.
    */ 


    //vec.iter==&vec 이다.
    let vec=vec![1,2,3];
    for val in vec.iter(){
        println!("{}",*val)
        //println!("{val}")-->애도된다. 자동 참조 기능이있음.
    }
    //d이렇게하면 vec2는 메모리 소유권을 vec2.into_iter()라는 반복자를 만들고 나서 기존의 vec2데이터에 대한 소유권을 소실-->즉 사라진다.
    //또한 넘기고 나서 그안의 요소요소를 수정하고싶으므로 mut val을 한다.
    // into_iter은 vec2의 value로 구성된 iteration을 반환한다.
    //만약 vec2.into_iter() 이거 자체를 수정가능한 반복자로 만들려면 val mut vec3=vec2.into_iter() 로 해주면된다.
    let vec2=vec![1,2,3];
    for mut val in vec2.into_iter(){
       val+=10;
       println!("{val}")     
    }

    // mut가 붙은 값에 대해서 원본의 각요소의 교체가 가능한 vec3의 변수들의 참조값 iteration을 넘겨준다.
    let mut vec3=vec![1,2,3];
    for val in vec3.iter_mut(){
        //레퍼런스값 가져온것이므로 *val로 접근
        *val*=2;
    }
    for val in vec3.iter(){
        println!("{}",*val)
        //println!("{val}")
    }

    //map은 최종적으로 다른 iteration을 반환 즉 원본을 바꾸고싶다면은 iter_mut~~foreach이런꼴로 바꾸는게 맞다.
    let  vec4=vec![1,2,3,4];

    //iter은 vec4의 각값들의 주소값을 넘긴다.
    //그리고 filter의 특징은 변수의 참조값을 넘긴다.
    //즉 해당 코드에서는 filter안에 현재 &&i32라는 vec4의 각요소들의 참조값의 참조값이 들어가있는상황
    //여기서 rust는 desconstruct란게 존재하는대
    //let y=&x 일경우 y에는 x의 참조값이 들어가나
    //let &z=y 일경우 y의 역참조 즉 x의 변수 값이 들어간다.
    //이걸 패턴 매칭이라는 러스트의 특징이다,
    //고로 현재 filter에서는 &&x로 클로저 참조로 역참조를 통해서 value를 가져와야 한다는것.

    //정리하자면 filter는 값의 변형이아닌 조건에따른 분기가 목적이기에 iteration값의 참조를 받아서 처리.->즉 vec4.iter()가 vec4의 각요소의 참조값을 들고있으므로 이 참조값의 참조를 받는다.
    //map, foreach는 iteration에서 넘겨준 값을 그냥 받아서 처리한다.
    //즉 필터로 걸러주면 map은 vec4 iteration으로 부터 해당 값에 들어있는 vec4안의 변수들의 참조값을 걍넘겨받는것
    //foreach는 map이 새로운 iteration을 넘기느대 그값들을 그냥 받아서 처리한다이말.
    vec4.iter().filter(|&&x| x%2==0)
    .map(|&x| x*2)
    .for_each(|x| println!("iteration method:{x}"));


    //takewhile은 filter하고 비슷한 역할을하며 애도 참조의 참조인대 애의 특징은 조건과 안맞는 원소를 만나면 바로중단한다.
    //즉 이예제는 출력이 1개만 나올것.
    vec4.iter().take_while(|&&x| x%2!=0)
    .for_each(|&x| println!("take while:{x}"));



    //filter map은  s.paser::<i32>()의 결과가 option일떄 애로부터 값만 뽑아내는 메서드이다.
    //즉 map의 결과가 option일때 거기서 none인애들 빼고 some으로부터 값만 추출한다 이렇게 생각하면됨.
    let a = ["1", "two", "NaN", "four", "5"];
    a.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .for_each(|x|println!("{x}"));


}