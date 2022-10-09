// 'T' type is referred to any data type: native types, compound types, structs, enums, etc.

// This structure allows points of any type: int, float, etc.
// This structure only allows x and y with the same type values
struct StaticPoint<T> {
    x:T,
    y:T,
}

// This structure allows x and y with different type values
struct DynamicPoint<T, V> {
    x:T,
    y:V,
}

fn main() {
    // Point base on floating point
    let point1 = StaticPoint {
        x: 2.5,
        y: 6.3,
    };

    // Point based on integers
    let point2 = StaticPoint {
        x: 3,
        y: 4,
    };

    // Point with different type values
    let point3 = DynamicPoint {
        x: 5.7,   // float
        y: 3,     // integer  
    };

    println!("Point1: ({}, {})", point1.x, point1.y);
    println!("Point2: ({}, {})", point2.x, point2.y);
    println!("Point3: ({}, {})", point3.x, point3.y);
}
