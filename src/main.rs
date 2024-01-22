use std::mem::zeroed;

fn main() {
    // hello_world()
    // variable_change()
    // variable_shadowing()
    basic_type();
    // float_type()
}

fn float_type(){

}
fn basic_type(){
    // 数值类型 i8 i16... u8 u16...f32 f64
    //字符串 字符串字面量和字符串切片 &str
    // bool
    // 字符类型 unicode
    // 单元类型 ()
    let _guess:i8="42".parse().expect("not a numer");
    /**
     整形字面量可以用下表的形式书写：
    数字字面量	示例
    十进制	    98_222
    十六进制	    0xff
    八进制	    0o77
    二进制	    0b1111_0000
    字节 (仅限于 u8)	b'A'
     */
    let x=98_222; //下划线 _ 只是为了提高可读性而存在,1000000可以写成1_000_000
    let y=0xff;
    let z=0o77;
    let v=0b1111_0000;
    let b:u8=b'A';
    println!("x is:--->{}",x);
    println!("y is:--->{}",y);
    println!("z is:--->{}",z);
    println!("v is:--->{}",v);
    println!("b is --->{}",b);
    /**
     关于溢出
    使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    如果使用 checked_* 方法时发生溢出，则返回 None 值
    使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    使用 saturating_* 方法使值达到最小值或最大值
     */
    let a:u8=255;
    let b=a.wrapping_add(20);
    println!("warpping方式 b is {}",b);
    let b=a.checked_add(20);
    println!("checked b is:{:?}",b);
    let b=a.overflowing_add(20);
    println!("overflow b is:{:?}",b);
    let b=a.saturating_add(20);
    println!("saturating b is {:?}",b);
}

fn variable_shadowing(){
    let x=5;
    let x=x+5; // 这里是生成了一个完全不同的变量，涉及内存的再分配，和mut不同，mut是修改同一个内存地址上的值，不会发生内存对象的再分配
    println!("x is {:?}",x);
    {
        println!("in block the x is {}",x);
        let x=x*2;
        println!("new x is {}",x);
    }
    println!("x is {}",x);
    let y:u8=8;
    let y:u8=y+9;
    println!("the y is {}",y);;
}
fn variable_change(){
    let x=5;
    println!("x is {}",x);
    let mut y =8;
    println!("y is {}",y);
    y=9;
    println!("change y to {}",y);
    let _z=9; //对于不使用的变量，前面要加上下划线
    //变量解构
    let (a,mut b):(bool,bool)=(true,false);
    println!("a is {:?},b is {:?}",a,b);
    // b=false;
    assert_eq!(a,b);
    const MAX_POINT:u32=89;
}

#[warn(dead_code)]
fn hello_world(){
    let ch="Ni Hao";
    let en="hello";
    let regin=[ch,en];
    // rust的集合类型不能直接循环，需要变成迭代器
    for r in regin.iter() {
        println!("{}",r)
    }

}