pub fn control_method(){
  //if문
  if 1>0{
    println!("true")
  }
  else{
    println!("false")
  }

  //match문 switch라고 생각하면됨.
  let test_val:i32=1;

  let ans=match test_val{
    1=>"true",
    _=>"false",
  };
  println!("ans:{ans}");


  //if let--> mathc문이면 match 분기 후에 따로 println을 써줘야됨.
  //to_digit는 option을 반환 option이뭔지는 후술.
  let c='5';
  if let Some(num)=c.to_digit(10){
    println!("num={}",num);
  }
  else{
    println!("num fail")
  }

  //for 문
  let mut sum=0;
  for i in 1..=100{
    sum+=i;
  }
  println!("sum={}",sum);


  //loop do while문에 해당된다.
  let mut i=0;
  loop{
    if 10>i{
      i+=1
    }
    else{
      break
    }
  }
  println!("i:{i}");

  i=0;

  while 10>i{
    i+=1
  }
 println!("i:{i}");
}
