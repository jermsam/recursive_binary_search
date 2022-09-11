fn main() {
    println!("Hello, world!");
    let sorted_list:[i8;10] = [0,1,2,3,4,5,6,7,8,9];
    let target:i8 = 11;
    verify(&sorted_list[..], &target,&recursive_binary_search);
    let target:i8 = 0;
    verify(&sorted_list[..], &target,&recursive_binary_search);
}

// ideal for short unsorted lists
fn recursive_binary_search (list: &[i8], target: &i8) -> Option<i8> {
    /* Returns index position if found else returns None */
    let length:usize = list.len();
    if length == 0 {
       return  None
    }
    let  midpoint:usize = length / 2;
    if *target < list[midpoint] {
       return recursive_binary_search(&list[..midpoint],&target)
    } else if  *target > list[midpoint]  {
        return recursive_binary_search(&list[(midpoint+1)..],&target)
    }else  {
        Some(midpoint as i8)
    }

}

fn verify(list:&[i8], target: &i8, search: &dyn Fn(&[i8], &i8) -> Option<i8>) {
    let index: Option<i8> = search(&list, &target);
    // match index {
    //     None => {
    //         println!("There is no match for {} in {:?}.", target,list);
    //     },
    //     Some(i) => {
    //         println!("Position of {} in {:?} is {}", target, list,i);
    //     },
    // }
    if let Some(i) = index {
        println!("Position of {} in {:?} is {}", target, list,i);
    } else {
        println!("There is no match for {} in {:?}.", target,list);
    }
}