struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn code(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    let green = RGB::code(0, 255, 0);
    println!("green is: {:?}", green);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
