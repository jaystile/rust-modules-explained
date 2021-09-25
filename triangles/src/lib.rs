#[cfg(test)]
pub mod triangles {

    use geometry::dims::{Type,Dimensional};

    #[test]
    fn validate_type_test() {
        let t = Triangle{};
        assert!(matches!(t.dimensions(), Type::D2));
    }

    struct Triangle {}

    impl Dimensional for Triangle {
        fn dimensions(&self) -> Type {
            return Type::D2;
        }
    }
}
