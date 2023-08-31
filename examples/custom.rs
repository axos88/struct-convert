use struct_convert::Convert;

#[derive(Debug,  PartialEq)]
struct B {
    bid: i64,
}

#[derive(Debug,  Convert, PartialEq)]
#[convert(into = "B")]
struct A {
    #[convert_field(rename = "bid", custom_fn = "str_to_i64")]
    id_str: String,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "B")]
struct C {
    #[convert_field(rename = "bid", custom_fn = "to_point")]
    point: Point,
}

struct D {
    str: String,
    bid: i64,
}

#[derive(Debug, Convert, PartialEq)]
#[convert(from = "D")]
#[convert(into = "D")]
struct E {
    str: String,
    #[convert_field(from = "D", rename = "bid", custom_fn = "to_point_from_d")]
    #[convert_field(into = "D", rename = "bid", custom_fn = "from_point_to_d")]
    point: Point

}

#[derive(Debug,  PartialEq)]
struct Point(i64, i64);

fn str_to_i64(a: &A) -> i64 {
    a.id_str.parse().unwrap()
}

fn to_point(b: &B) -> Point {
  Point(b.bid, b.bid)
}

fn to_point_from_d(d: &D) -> Point {
    Point(d.bid, d.bid)
}

fn from_point_to_d(e: &E) -> i64 { e.point.0 }

fn main() {

}

#[test]
fn test_custom() {
    let a = A { id_str: "4".into() };
    let b: B = a.into();
    debug_assert_eq!(B { bid: 4 }, b);
    let c: C = b.into();
    debug_assert_eq!(C { point: Point(4, 4) }, c);

    let d = D { str: "str".into(), bid: 42 };
    let e: E = d.into();
    let d2: D = e.into();
    debug_assert_eq!(E { str: "str".into(), point: Point(42, 42) }, e);
    debug_assert_eq!(D { str: "str".into(), bid: 42 }, d2);
}
