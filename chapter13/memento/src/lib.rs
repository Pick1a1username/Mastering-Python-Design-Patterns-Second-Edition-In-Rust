pub mod pseudo_pickle {

    pub struct Pickle<T: Clone> {
        obj: T,
    }

    impl<T> Pickle<T>
    where T: Clone
    {
        fn clone(&self) -> T {
            self.obj.clone()
        }
    }

    pub fn dumps<T: Clone>(obj: T) -> Pickle<T> {
        Pickle { obj: obj }
    }

    pub fn loads<T>(obj: Pickle<T>) -> T
    where T: Clone
    {
        return obj.clone();
    }
}


#[cfg(test)]
mod pseudo_pickle_tests {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct TestStruct {
        attr0: i32,
        attr1: String,
    }

    #[test]
    fn fn_dumps() {
        let test_struct_0 = TestStruct { attr0: 1, attr1: "test_string".to_string() };
        let test_struct_0_memento = pseudo_pickle::dumps(test_struct_0.clone());
        assert_eq!(test_struct_0, pseudo_pickle::loads(test_struct_0_memento));
    }
}

