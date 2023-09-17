// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word, Some(target));
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // You can think of this as a stack of options, where None means the
        // stack is empty.
        // If you need to, revise the section on `while let` and `if let` in
        // https://doc.rust-lang.org/book/ch06-03-if-let.html
        // You can also use `flatten` as shown in the section on `if let`:
        // https://doc.rust-lang.org/book/ch06-03-if-let.html#multiple-patterns
        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(cursor, integer);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
