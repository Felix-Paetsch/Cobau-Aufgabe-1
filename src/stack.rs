use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        Vec::new()
    }

    fn push_val(&mut self, val: i32) {
        self.push(val);
    }

    fn top_val(&self) -> Option<&i32> {
        self.last()
    }

    fn pop_val(&mut self) -> Option<i32> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}


#[derive(Debug)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        ListStack::Nil
    }

    fn push_val(&mut self, val: i32) {
        match self {
            ListStack::Val(value, next) => {
                let new_next = Some(Box::new(ListStack::Val(*value, next.take())));
                *self = ListStack::Val(val, new_next);
            }
            ListStack::Nil => {
                *self = ListStack::Val(val, Some(Box::new(ListStack::Nil)));
            }
        }
    }

    fn top_val(&self) -> Option<&i32> {
        match self {
            ListStack::Val(value, _) => Some(value),
            ListStack::Nil => None,
        }
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            ListStack::Val(value, next) => {
                let old_value = *value;
                let next_val = next.take().unwrap();
                *self = *next_val;
                Some(old_value)
            }
            ListStack::Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            ListStack::Val(_, _) => false,
            ListStack::Nil => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;

    #[test]
    fn vec_fill_and_clear() {
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    #[test]
    fn linked_fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }
}
