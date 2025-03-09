use op_proc::array;
fn main() {
    let arr = [2, 3, 4, 5, 6, 7, 877];
    let arr = array!(3, i, { arr[i + 3] + 1 });
    assert_eq!(arr, [6, 7, 8]);
}
