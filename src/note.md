# Rust学习笔记

## 1、循环

### 一些疑问

1、loop 循环由break退出，break后可以跟表达式返回，表达式既可以有分号，也可以没有分号，这个返回的特性是break和loop的特有还是rust的

>Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.

这个特性决定的，摘自书的3.3节，同理还有函数的返回。

2、以下代码：

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
break 后面加不加`;`效果一样

3、以下代码：

```
fn main() {
    let mut number = 3;

    let result = while number != 0 {
        println!("{}!", number);

        number = number - 1
    };

    println!("LIFTOFF!!!{:?}",result);
}
```
`number = number -1`这句话可以加;号也可以不加，result都没有值，但最后不可以是一个表达式。比如x+1之类的

