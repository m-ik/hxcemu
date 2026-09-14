pub struct Alu;

impl Alu {
    pub fn compute(
        x: u16,
        y: u16,
        zx: bool,
        nx: bool,
        zy: bool,
        ny: bool,
        f: bool,
        no: bool,
    ) -> (u16, bool, bool) {
        let x = if zx { 0 } else { x };
        let x = if nx { !x } else { x };
        let y = if zy { 0 } else { y };
        let y = if ny { !y } else { y };
        let out = if f { x.wrapping_add(y) } else { x & y };
        let out = if no { !out } else { out };
        let zr = out == 0;
        let ng = (out & 0x8000) != 0;

        (out, zr, ng)
    }
}

// zx|nx|zy|ny| f|no|out
// --+--+--+--+--+--+---
//  1| 0| 1| 0| 1| 0|  0
//  1| 1| 1| 1| 1| 1|  1
//  1| 1| 1| 0| 1| 0| -1
//  0| 0| 1| 1| 0| 0|  x
//  1| 1| 0| 0| 0| 0|  y
//  0| 0| 1| 1| 0| 1| !x
//  1| 1| 0| 0| 0| 1| !y
//  0| 0| 1| 1| 1| 1| -x
//  1| 1| 0| 0| 1| 1| -y
//  0| 1| 1| 1| 1| 1|x+1
//  1| 1| 0| 1| 1| 1|y+1
//  0| 0| 1| 1| 1| 0|x-1
//  1| 1| 0| 0| 1| 0|y-1
//  0| 0| 0| 0| 1| 0|x+y
//  0| 1| 0| 0| 1| 1|x-y
//  0| 0| 0| 1| 1| 1|y-x
//  0| 0| 0| 0| 0| 0|x&y
//  0| 1| 0| 1| 0| 1|x|y

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, false, true, false, true, false);
        assert_eq!(out, 0);
        assert!(zr);
        assert!(!ng);
    }

    #[test]
    fn one() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, true, true, true, true);
        assert_eq!(out, 0x1);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn minus_one() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, true, false, true, false);
        assert_eq!(out, 0xffff);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn x() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, true, true, false, false);
        assert_eq!(out, 0xde);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, false, false, false, false);
        assert_eq!(out, 0xad);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn not_x() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, true, true, false, true);
        assert_eq!(out, 0xff21);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn not_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, false, false, false, true);
        assert_eq!(out, 0xff52);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn neg_x() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, true, true, true, true);
        assert_eq!(out, 0xff22);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn neg_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, false, false, true, true);
        assert_eq!(out, 0xff53);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn x_dec() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, true, true, true, false);
        assert_eq!(out, 0xdd);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn y_dec() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, true, true, false, false, true, false);
        assert_eq!(out, 0xac);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn x_plus_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, false, false, true, false);
        assert_eq!(out, 0x18b);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn x_minus_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, true, false, false, true, true);
        assert_eq!(out, 0x31);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn y_minus_x() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, false, true, true, true);
        assert_eq!(out, 0xffcf);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn x_and_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, false, false, false, false, false);
        assert_eq!(out, 0x8c);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn x_or_y() {
        let (out, zr, ng) = Alu::compute(0xde, 0xad, false, true, false, true, false, true);
        assert_eq!(out, 0xff);
        assert!(!zr);
        assert!(!ng);
    }
}
