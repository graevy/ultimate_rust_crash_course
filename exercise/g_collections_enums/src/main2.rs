struct Point {
    x:i32,
    y:i32
}

struct TuplePoint(i32, i32);

enum NotPoint {
    EnumTypeName(bool),

}


enum MyResult<T, E>
    where E: std::error::Error
{
    Ok(T),
    Err(MyResult)
}

fn main() {}