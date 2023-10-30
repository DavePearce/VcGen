use crate::{MachineWord};

impl MachineWord for u8 {
    fn less_than(self,rhs:Self)->Self {
        if self < rhs { 1 } else { 0 }
    }
    fn equal(self,rhs:Self)->Self {
        if self == rhs { 1 } else { 0 }
    }
    // Arithmetic
    fn add(self,rhs:Self)->Self {
        self.wrapping_add(rhs)
    }
    fn mul(self,rhs:Self)->Self {
        todo!();
    }
    fn div(self,rhs:Self)->Self {
        todo!();
    }
    fn rem(self,rhs:Self)->Self {
        todo!();
    }
    fn neg(self)->Self {
        todo!();
    }
    // Bitwise
    fn and(self,rhs:Self)->Self {
        todo!();
    }
    fn or(self,rhs:Self)->Self {
        todo!();
    }
    fn xor(self,rhs:Self)->Self {
        todo!();
    }
    fn not(self)->Self {
        todo!();
    }
}
