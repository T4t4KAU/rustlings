#[rustfmt::skip]
fn main() {
    // 定义一个 Option<()> 类型的变量并初始化为 None
    let my_option: Option<()> = None;
    // 检查 Option 是否为 Some 类型，如果是则打印其值
    if let Some(value) = my_option {
        println!("The value inside Option is: {:?}", value);
    } else {
        println!("The Option is None.");
    }

    // 定义一个数组切片
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // 将向量的大小调整为 0，使其变为空向量
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换 value_a 和 value_b 的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");

    // 可以添加更多操作，例如对向量添加元素
    my_vec.push(10);
    println!("The vector after adding an element: {my_vec:?}");
}