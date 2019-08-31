pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}
// パブリックにした関数　中身は未実装

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}
// プライベート関数　中身は未実装

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
// プライベート関数　中身は未実装
