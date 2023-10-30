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
    use hw_procedure_macros::my_proc_macro;
    use super::*;

    fn foo() -> i32 {
        55_i32
    }

    fn bar() -> i32 {
        77_i32
    }

    fn baz() -> String{
        String::from("some string")
    }

    fn fo() -> String{
        String::from("fo string")
    }

    fn fooo() -> String{
        String::from("fooo string")
    }

    #[test]
    fn test_my_macro() {
        assert_eq!((), my_macro!());
        assert_eq!((String::from("some string"), 55, 77), my_macro!(baz(), foo(), bar()));
    }

    #[test]
    fn test_my_proc_macro() {

        my_proc_macro!("fo", "foo", "fooo");


        // assert_eq!(("fo"),  my_proc_macro!("fo()"));
        //assert_eq!(("fo", "fooo"),  my_proc_macro!("fo", "fooo"));
        // assert_eq!(("fo", "fooo"),  my_proc_macro!(""fo"", ""foo"", ""fooo""));
    }

}
