// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    //answer 1
    // let vec0 = Vec::new();
    // let mut vec1 = fill_vec(vec0.clone());

    //2
    // let vec0 = Vec::<i32>::new();
    // let mut vec1 = fill_vec_ref(&vec0);

    //3 // output is diffent because vec0 is being filled in the function, wheras the other it is being cloned before the filling
    let mut vec0: Vec<i32> = Vec::new();
    let mut vec1 = fill_vec_mut_borrow(&mut vec0);


    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_ref(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_internal_clone(vec: Vec<i32>) -> Vec<i32> {
    // didn't work due to borrowing stuff. I think I'm getting this now.
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_mut_borrow(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()
}
