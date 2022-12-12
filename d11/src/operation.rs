// operation.evaluate

struct Operation {
    operation: OperationType,
    right: OperationParameter,
}

enum OperationParameter {
    Constant(u32),
    Old,
}

impl Operation {
    fn evaluate(&self, old: u32) -> u32 {
        // Version 1
        let right = match self.right {
            OperationParameter::Constant(c) => c,
            OperationParameter::Old => old,
        };

        match &self.operation {
            OperationType::Add => old + right,
            OperationType::Multiply => old * right,
        };

        // Version 2
        match (&self.operation, &self.right) {
            (OperationType::Add, OperationParameter::Constant(c)) => old + c,
            (OperationType::Add, OperationParameter::Old) => old + old,
            (OperationType::Multiply, OperationParameter::Constant(c)) => old * c,
            (OperationType::Multiply, OperationParameter::Old) => old * old,
        };

        // Version 3
        match self.operation {
            OperationType::Add => {
                old + match self.right {
                    OperationParameter::Constant(c) => c,
                    OperationParameter::Old => old,
                }
            }
            OperationType::Multiply => {
                old * match self.right {
                    OperationParameter::Constant(c) => c,
                    OperationParameter::Old => old,
                }
            }
        }
    }
}
