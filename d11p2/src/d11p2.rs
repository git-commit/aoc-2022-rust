//
// insert were items gorillas meme here
//
// Desired output: monkey business
//
// monkey business is calculated by multiplying the inspections of the two most active monkeys over
// 20 rounds of chaos.
//
// A round of the monkey business goes like this:
// 1. Inspect an item
// 2. Be relieved that the monkey didn't break your stuff: floor(divide worry level by 3)
// 3. Monkey tests your worry level and throws all items according to the test
//
// Notes:
// * Monkeys act in the order given in the input
// * Thrown items go on the end of the monkeys item list
// * Empty monkey be bored and doesn't do anything (but note that a monkey might get an item during
//   a round)
// * The number of monkeys is not fixed
//
// Simplifications:
// * There's no need to track the inspections per item, only the inspections per monkey
// * Tests are only whether the worry level is divisible by a number
// * an operation always starts with the old value
// * positive vibes (read integers) only
//
// Complications:
// * Operations can reference the old value twice (e.g. Operation: new = old * old)
// * worry levels are now not divided by 3 anymore
//
// Solution:
// * by multiplying all the divisors of the tests, we get a number that is divisible by all the
//   divisors of the tests and thus we can use the modulo operator on the worry levels with this
//   number while mathematically getting the same result as without the modulo operator
// * we need to use a larger type than u32 still though

use itertools::Itertools;

type NumT = u128;

#[derive(Default)]
struct Djungle {
    monkeys: Vec<Monkey>,
    mod_factor: NumT,
}
impl Djungle {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let mod_factor = monkeys
            .iter()
            .map(|m| m.test.divisor.clone())
            .fold(1u32.into(), |acc: NumT, d| acc * d);
        Djungle {
            monkeys,
            mod_factor,
        }
    }
}

trait Parsable {
    fn parse(input: String) -> Self;
}

struct Monkey {
    id: usize,
    items: Vec<Item>,
    inspections: NumT,
    test: Test,
    operation: Operation,
}

impl Monkey {
    fn inspect_and_throw(&mut self, mod_factor: &NumT) -> Vec<Throw> {
        // each item will be inspected
        self.inspections = self.inspections.clone() + self.items.len() as NumT;
        // be releived that he didn't break the item
        self.items
            .iter_mut()
            .for_each(|i| i.worry_level = i.worry_level.clone() % mod_factor);
        self.items
            .iter_mut()
            .for_each(|i| i.apply_operation(&self.operation));
        // generate throws
        let throws = self
            .items
            .iter()
            .map(|i| self.test.throw(i.clone()))
            .collect();
        self.items.clear();
        throws
    }

    fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
}
struct Test {
    divisor: NumT,
    true_target: usize,
    false_target: usize,
}
impl Test {
    fn throw(&self, item: Item) -> Throw {
        if &item.worry_level % &self.divisor == 0u32.into() {
            Throw {
                item: item.clone(),
                target: self.true_target,
            }
        } else {
            Throw {
                item: item.clone(),
                target: self.false_target,
            }
        }
    }
}
impl Parsable for Test {
    fn parse(input: String) -> Self {
        // Tests have three lines
        assert!(
            input.lines().count() == 3,
            "Input for Test is fucked: {}",
            input
        );

        // parse the first line as divisor
        // Test: divisible by 23
        // into 23
        let divisor = input
            .lines()
            .nth(0)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<NumT>()
            .unwrap();

        // parse the second line as true target
        //     If true: throw to monkey 2
        // into 2
        let true_target = input
            .lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // parse the third line as false target
        //     If false: throw to monkey 3
        // into 3
        let false_target = input
            .lines()
            .nth(2)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Test {
            divisor,
            true_target,
            false_target,
        }
    }
}

struct Throw {
    item: Item,
    target: usize,
}
struct Operation {
    operation: OperationType,
    right: OperationParameter,
}

enum OperationParameter {
    Constant(NumT),
    Old,
}

impl Operation {
    fn evaluate(&self, old: &NumT) -> NumT {
        let right = match &self.right {
            OperationParameter::Constant(c) => c.clone(),
            OperationParameter::Old => old.clone(),
        };

        match &self.operation {
            OperationType::Add => old + right,
            OperationType::Multiply => old * right,
        }
    }
}
impl Parsable for Operation {
    fn parse(input: String) -> Operation {
        let operation = match input.find(|op| op == '*' || op == '+') {
            Some(pos) => match input.chars().nth(pos) {
                Some('*') => OperationType::Multiply,
                Some('+') => OperationType::Add,
                _ => panic!("Operation is not available"),
            },
            _ => panic!("Operation not found"),
        };

        let (_, right_unparsed) = input.split_once(|op| op == '*' || op == '+').unwrap();
        let right = match right_unparsed.trim() {
            "old" => OperationParameter::Old,
            c => OperationParameter::Constant(c.parse::<NumT>().unwrap()),
        };
        Operation { operation, right }
    }
}
enum OperationType {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    worry_level: NumT,
}

impl Item {
    fn apply_operation(&mut self, operation: &Operation) {
        self.worry_level = operation.evaluate(&self.worry_level);
    }
}

impl Djungle {
    fn monkey_business(&mut self) -> NumT {
        self.monkeys
            .iter()
            .map(|m| &m.inspections)
            .sorted()
            .rev()
            .take(2)
            .fold(1u32.into(), |a, b| a * b)
    }

    fn round(&mut self) {
        // self.monkeys.iter_mut().for_each(|m| {
        //     let t = m.inspect_and_throw();
        //     t.iter().for_each(|t|self.monkeys[t.target].items.push(t.item));
        // });
        for monkey_id in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[monkey_id];
            let throws = monkey.inspect_and_throw(&self.mod_factor);
            for throw in throws {
                let target_monkey = &mut self.monkeys[throw.target];
                target_monkey.catch(throw.item);
            }
        }
        println!(
            "Items for Monkey: {:?}\nInspections: {:?}",
            self.monkeys
                .iter()
                .map(|m| (&m.items)
                    .into_iter()
                    .map(|i| &i.worry_level)
                    .collect_vec()
                    .to_owned())
                .collect::<Vec<Vec<&NumT>>>(),
            self.monkeys.iter().map(|m| &m.inspections).collect_vec()
        );
    }
}

impl Parsable for Djungle {
    fn parse(input: String) -> Self {
        let mut monkeys = Vec::new();
        let mut lines = input.lines();
        let lines_per_monkey = 7;
        let lines_in_input = input.lines().count() + 1;
        println!("Lines in input: {}", lines_in_input);
        let monkey_count = lines_in_input / lines_per_monkey;
        for monkey_id in 0..monkey_count {
            // skip first line that contains the id
            lines.next();
            // parse items
            //   Starting items: 79, 98
            // into: [79, 98]
            let items = lines
                .next()
                .unwrap()
                .split(":")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|i| Item {
                    worry_level: i.trim().parse::<NumT>().unwrap(),
                })
                .collect();
            // parse operation
            let operation = Operation::parse(lines.next().unwrap().to_string());
            // parse next three lines as test
            let test = Test::parse(
                lines
                    .by_ref()
                    .take(3)
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            // skip empty line
            lines.next();

            monkeys.push(Monkey {
                id: monkey_id,
                items,
                inspections: 0u32.into(),
                operation,
                test,
            });
        }
        Djungle::new(monkeys)
    }
}

fn main() {
    // Read input.txt into a vector of strings
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut djungle = Djungle::parse(input);
    for _ in 0..10000 {
        djungle.round();
    }
    println!("{}", djungle.monkey_business());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn djungle_fixture() -> Djungle {
        Djungle::new(vec![
            Monkey {
                id: 0,
                inspections: 0u32.into(),
                items: vec![79, 98]
                    .iter()
                    .map(|i| -> NumT { (*i as u32).into() })
                    .map(|i| Item { worry_level: i })
                    .collect(),
                operation: Operation {
                    operation: OperationType::Multiply,
                    right: OperationParameter::Constant(19u32.into()),
                },
                test: Test {
                    divisor: 23u32.into(),
                    true_target: 2,
                    false_target: 3,
                },
            },
            Monkey {
                id: 1,
                inspections: 0u32.into(),
                items: vec![54, 65, 75, 74]
                    .iter()
                    .map(|i| -> NumT { (*i as u32).into() })
                    .map(|i| Item { worry_level: i })
                    .collect(),
                operation: Operation {
                    operation: OperationType::Add,
                    right: OperationParameter::Constant(6u32.into()),
                },
                test: Test {
                    divisor: 19u32.into(),
                    true_target: 2,
                    false_target: 0,
                },
            },
            Monkey {
                id: 2,
                inspections: 0u32.into(),
                items: vec![79, 60, 97]
                    .iter()
                    .map(|i| -> NumT { (*i as u32).into() })
                    .map(|i| Item { worry_level: i })
                    .collect(),
                operation: Operation {
                    operation: OperationType::Multiply,
                    right: OperationParameter::Old,
                },
                test: Test {
                    divisor: 13u32.into(),
                    true_target: 1,
                    false_target: 3,
                },
            },
            Monkey {
                id: 3,
                inspections: 0u32.into(),
                items: vec![74]
                    .iter()
                    .map(|i| -> NumT { (*i as u32).into() })
                    .map(|i| Item { worry_level: i })
                    .collect(),
                operation: Operation {
                    operation: OperationType::Add,
                    right: OperationParameter::Constant(3u32.into()),
                },
                test: Test {
                    divisor: 17u32.into(),
                    true_target: 0,
                    false_target: 1,
                },
            },
        ])
    }

    #[test]
    fn test_first_round_of_monkey_0() {
        let mut dj = djungle_fixture();
        dj.round();
        assert_eq!(
            dj.monkeys[0].items,
            vec![20, 23, 27, 26]
                .iter()
                .map(|i| -> NumT { (*i as u32).into() })
                .map(|i| Item {
                    worry_level: i.to_owned()
                })
                .collect::<Vec<Item>>()
        );
    }
    #[test]
    fn test_first_round_of_monkey_1() {
        let mut dj = djungle_fixture();
        dj.round();
        assert_eq!(
            dj.monkeys[1].items,
            vec![2080, 25, 167, 207, 401, 1046]
                .iter()
                .map(|i| -> NumT { (*i as u32).into() })
                .map(|i| Item {
                    worry_level: i.to_owned()
                })
                .collect::<Vec<Item>>()
        );
    }
    #[test]
    fn test_first_round_of_monkey_2() {
        let mut dj = djungle_fixture();
        dj.round();
        assert_eq!(dj.monkeys[2].items, vec![]);
    }
    #[test]
    fn test_first_round_of_monkey_3() {
        let mut dj = djungle_fixture();
        dj.round();
        assert_eq!(dj.monkeys[3].items, vec![]);
    }

    #[test]
    fn test_inspections() {
        let mut dj = djungle_fixture();
        let expected_inspections: Vec<NumT> =
            vec![2u32.into(), 4u32.into(), 3u32.into(), 6u32.into()];
        dj.round();
        let actual_inspections = dj
            .monkeys
            .iter()
            .map(|m| m.inspections.clone())
            .collect::<Vec<NumT>>();
        assert_eq!(actual_inspections, expected_inspections);
    }

    #[test]
    fn test_monkey_business() {
        let mut dj = djungle_fixture();
        for _ in 0..10000 {
            dj.round();
        }
        assert_eq!(dj.monkey_business(), 2713310158u32.into())
    }
}
