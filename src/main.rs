#![allow(dead_code)]
#![allow(unused_variables)]

use movies::english::comedy;

mod movies {
    pub mod english {
       pub mod comedy {
        
          pub  fn play(name: String) {
                println!("{}",name );
            }
        }  

    }

}

fn main() {

    comedy::play(String::from("superbad"));

}
