struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let mut f = Foo { x: &2 };  

    {
        test_life_time(&mut f);      
    }
      
    println!("{}", f.x);    
}  

fn test_life_time<'a>( f : &mut Foo<'a> )
{

    let y : &'a i32 = &Box::new(5);      
    f.x = y;
    //    println!("{}", f.x);
      
}