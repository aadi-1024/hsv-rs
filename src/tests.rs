#[cfg(test)]
mod tests {
    
    #[test]
    fn hsv_new() {
        use crate::color::Hsv;
        let c1 = Hsv::new(120.0, 0.5, 0.5);
        let c2 = Hsv::new(480.0, -1.5, 1.5);
        assert_eq!(c1.get_components(), c2.get_components());
    }

    #[test]
    fn map() {
        use crate::math::map;
        assert_eq!(map(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map(5.0, 0.0, 10.0, 10.0, 100.0), 45.0);
    }

    #[test]
    fn hex_4() {
        use crate::math::to_hex;
        assert_eq!(to_hex(3), "3");
        assert_eq!(to_hex(17), "11");
        assert_eq!(to_hex(63), "3F");
        assert_eq!(to_hex(255), "FF");
    }
}
