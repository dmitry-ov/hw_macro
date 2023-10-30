/*
Декларативный макрос, который принимает список имён функций и возвращает
кортеж из возвращаемых функциями значений. Число функций может быть произвольным.
Пример:
let (foo_result, bar_result, bas_result) = my_macro!(foo, bar, baz);
*/

#[macro_export]
macro_rules! my_macro {
    ($($func:expr), *) => {
        ($($func), *)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hw_procedure_macros::my_proc_macro;

    fn foo() -> i32 {
        55_i32
    }

    fn bar() -> i32 {
        77_i32
    }

    fn baz() -> String {
        String::from("some string")
    }

    fn fo() -> String {
        String::from("fo")
    }

    fn fooo() -> String {
        String::from("fooo")
    }

    #[test]
    fn test_my_macro() {
        assert_eq!((), my_macro!());
        assert_eq!(
            (String::from("some string"), 55, 77),
            my_macro!(baz(), foo(), bar())
        );
    }

    #[test]
    fn test_my_proc_macro() {
        assert_eq!((), my_proc_macro!("foo"));
        assert_eq!(
            (String::from("fo"), String::from("fooo")),
            my_proc_macro!("fo", "foo", "fooo")
        );
    }
}
