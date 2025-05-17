struct Counter {
    count: i32,
}

enum CounterOperation {
    Increment,
    Decrement,
    Reset,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn apply_operation(&mut self, operation: CounterOperation) {
        match operation {
            CounterOperation::Increment => self.count += 1,
            CounterOperation::Decrement => self.count -= 1,
            CounterOperation::Reset => self.count = 0,
        }
    }
}

fn main() {
    let mut test_counter = Counter::new();
    test_counter.apply_operation(CounterOperation::Increment);
    test_counter.apply_operation(CounterOperation::Reset);
    test_counter.apply_operation(CounterOperation::Decrement);
    println!("count: {}", test_counter.count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_operations() {
        let mut counter = Counter::new();

        counter.apply_operation(CounterOperation::Increment);
        assert_eq!(counter.count, 1);

        counter.apply_operation(CounterOperation::Increment);
        assert_eq!(counter.count, 2);

        counter.apply_operation(CounterOperation::Decrement);
        assert_eq!(counter.count, 1);

        counter.apply_operation(CounterOperation::Reset);
        assert_eq!(counter.count, 0);

        counter.apply_operation(CounterOperation::Decrement);
        assert_eq!(counter.count, -1);
    }
}
