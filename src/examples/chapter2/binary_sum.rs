// !! NEED TO REFACTOR THIS CODE
// * AUTOMATICAL CALLY
// * UN-OBVIOUSLY INITIALIZATION OF RESULT

#[derive(Debug, PartialEq)]
pub enum BinaryElem {
    Zero,
    One,
}

impl BinaryElem {
    fn add<'a>(&'a self, other: &'a Self) -> (BinaryElem, bool) {
        match other {
            BinaryElem::One => {
                match self {
                    BinaryElem::One => {
                        return (BinaryElem::Zero, true)
                    },
                    BinaryElem::Zero => {
                        return (BinaryElem::One, false)
                    }
                }
            },
            BinaryElem::Zero => {
                match self {
                    BinaryElem::One => {
                        return (BinaryElem::One, false)
                    },
                    BinaryElem::Zero => {
                        return (BinaryElem::Zero, false)
                    }
                }
            }
        }
    }
}

pub fn example_2_1_4(c: &mut Vec<BinaryElem>, a: &Vec<BinaryElem>, b: &Vec<BinaryElem>){
    // push dummy data in front
    let mut carry = BinaryElem::Zero;
    for i in 0..a.len() {
        let index = (a.len() - 1) - i;
        let a = a.get(index).unwrap();
        let b = b.get(index).unwrap();
        let (c_index, is_carry_ab) = a.add(b);
        let (c_value, is_carry_c) = c_index.add(&carry); // before carried
        c[index+1] = c_value;
        if is_carry_ab || is_carry_c {
            carry = BinaryElem::One
        }
    }
    c[0] = carry;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example_2_1_4_01() {
        let a = vec![BinaryElem::One, BinaryElem::Zero, BinaryElem::One, BinaryElem::One]; // 1011
        let b = vec![BinaryElem::One, BinaryElem::One, BinaryElem::One, BinaryElem::One]; //  1111
        let mut c = vec![BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero]; // initialize
        let expected = vec![BinaryElem::One, BinaryElem::One, BinaryElem::Zero, BinaryElem::One, BinaryElem::Zero]; // expected
        example_2_1_4(&mut c, &a, &b); // c expect 11010
        assert_eq!(expected, c);
    }

    #[test]
    fn run_example_2_1_4_02() {
        let a = vec![BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero]; // 1011
        let b = vec![BinaryElem::One, BinaryElem::One, BinaryElem::One, BinaryElem::One]; //  1111
        let mut c = vec![BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero, BinaryElem::Zero]; // initialize
        let expected = vec![BinaryElem::Zero, BinaryElem::One, BinaryElem::One, BinaryElem::One, BinaryElem::One];
        example_2_1_4(&mut c, &a, &b); // c expect 11010
        assert_eq!(expected, c);
    }
}
