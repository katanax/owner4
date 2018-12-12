#![allow(unused_variables)]
fn main() {
    let mut s1 = String::from("STRING 01");

    {
        // test 1:	
        let s2 = &s1;
        let s3 = &s1;
        //let s4 = &mut s1;  // <- this will be error
    }

    {
        // test 2:	
        let s4 = &mut s1;
        //let s3 = &s1;  // <- this will be error
    }

    {
        // test 3:	
        {
            let s4 = s1;
            let s2 = &s4;
        }
    }
}
