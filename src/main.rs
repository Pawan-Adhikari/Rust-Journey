fn loop_and_panic(vec : Vec<i32>)
{
    for num in vec
    {
        if num < 0
        {
            panic!("Negative number found; crashing the program");
        }
        println!("{}",num);
    }
}

fn main()
{
    let numbers = vec![1,2,3,4,-5];
    loop_and_panic(vec![1,2,3,4,-5]);
}