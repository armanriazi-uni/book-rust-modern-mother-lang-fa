#![allow(dead_code, unused_variables)]

use std::slice;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-btree_bin --bin packtpub-btree-main```
///
/// ```cargo doc  --package packtpub-btree_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-btree_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore
use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
//use money_typesafe::currencies::{Money,GBP};


// fn adjust(ac:&mut BTreeMap<String,Money<GBP>>,name:&str,amount:Money<GBP>){
//     match ac.get_mut(name) {
//         Some(cval)=>{
//             *cval += amount;
//         },
//         None=>{
//             ac.insert(name.to_string(),amount);
//         }
//     }
// }

fn main() {
    // let mut accounts = BTreeMap::new();

    // let mut file = File::open("test_data/payments.ts").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data);

    // for s in data.split("\n"){
    //     let v:Vec<&str>= s.split(':').collect();
    //     println!("{:?}",v);
    //     if v.len() != 3 { println!("not 3 but {}",s.len());continue }

    //     match v[2].parse::<Money<GBP>>() {
    //         Ok(m)=>{
    //             adjust(&mut accounts,v[0],-m);
    //             adjust(&mut accounts,v[1],m);
    //             println!("OK:{}",m);
    //         },
    //         Err(e)=>{
    //             println!("Could not parse transaction {}, Err: {:?}",s,e);
    //         }
    //     }

    // }

    // println!("Final Accounts : {:?}",accounts);

    unimplemented!();

}
