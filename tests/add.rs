mod test1 {
    use simple_math::operations::addition::add;

    #[test]
    fn add_give_correct_outcome() {
        assert_eq!(4, add(2, 2))
    }
}

mod test2 {
    use simple_math::operations::addition::add;

    #[test]
    fn add_give_correct_outcome() {
        assert_eq!(6, add(3, 3))
    }
}

mod test3 {
    use simple_math::operations::addition::add;

    #[test]
    fn add_give_correct_outcome() {
        assert_eq!(42, add(40, 2))
    }
}
