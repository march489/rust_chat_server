fn increment_array(mut arr: [u8; 10]) -> [u8; 10] {
    // TODO: add one to every element of the array "arr", then return the array.

    // // Using mutable iterator
    // for mut x in arr.iter_mut() {
    //     *x = *x + 1;
    // }

    // // using map
    // arr.iter_mut().map(|x| *x += 1).collect::<Vec<()>>();

    // more idiomatic according to online sources
    // iterates over references
    for i in &mut arr {
        *i += 1;
    }

    // // another method
    // // iterates over indices
    // for i in 0..10 {
    //     arr[i] = arr[i] + 1;
    // }

    // NB: Even though we passed a mutable array,
    // what we got was a mutable copy, not a reference to the original
    // so we only mutate the array within function scope
    // the original array passed to this function is unchanged.
    arr
}

fn increment_array_ref(arr_ref: &mut [u8; 10]) {
    // TODO: add one to every element of the array "arr".
    // Nothing to return.

    // mutates in place
    // NB: Here we pass a reference

    // // Using map
    // arr_ref.iter_mut().map(|x| *x = *x + 1).collect::<Vec<()>>();

    // // using a for loop
    // for r in arr_ref {
    //     *r += 1;
    // }

    // // Using .iter_mut()
    // for r in arr_ref.iter_mut() {
    //     *r += 1;
    // }

    // NB: x is a mutable *REFERENCE* to the elements of the array
    for x in arr_ref {
        *x += 1;
    }
}

fn increment_slice(slice: &mut [u8]) {
    // TODO: add one to every element of the array "arr".
    // Nothing to return.
    for x in slice {
        *x += 1;
    }
}

fn main() {
    let mut array: [u8; 10] = [4, 5, 6, 7, 8, 9, 5, 5, 6, 10];
    dbg!(array);
    dbg!(increment_array(array));
    dbg!(array);
    dbg!(increment_array_ref(&mut array));
    dbg!(array);
    dbg!(increment_slice(&mut array));
    dbg!(array);
}
