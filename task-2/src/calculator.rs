///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task 
/// to practice enums, structs, and methods.
/// 
/// Complete the implementation of the Calculator struct and its methods.
/// 
/// The calculator should support basic arithmetic 
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history 
/// of operations.
/// 
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
/// 
///-------------------------------------------------------------------------------

#[derive(Clone)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication
}

impl OperationType {
    // TODO: Return the string representation of the operation sign
    // Addition -> "+", Subtraction -> "-", Multiplication -> "*"
    pub fn get_sign(&self) -> &str {
        //todo!()
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }
    
    // TODO: Perform the operation on two i64 numbers with overflow protection
    // Return Some(result) on success, None on overflow
    //
    // Example: OperationType::Multiplication.perform(x, y)
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        //todo!()
         match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Subtraction => x.checked_sub(y),
            OperationType::Multiplication => x.checked_mul(y),
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType
}

impl Operation {
    // TODO: Create a new Operation with the given parameters
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        //todo!()
         Operation {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>
}

impl Calculator {
    // TODO: Create a new Calculator with empty history
    pub fn new() -> Self {
        //todo!()
        Calculator {
            history: Vec::new(),
        }
    }
    
    // TODO: Perform addition and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        //todo!()
        let result = OperationType::Addition.perform(x, y);
        let op = Operation::new(x, y, OperationType::Addition);
        self.history.push(op);
        result
    }
    
    // TODO: Perform subtraction and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        //todo!()
        let result = OperationType::Subtraction.perform(x, y);
        let op = Operation::new(x, y, OperationType::Subtraction);
        self.history.push(op);
        result
    }
    
    // TODO: Perform multiplication and store successful operations in history
    // Return Some(result) on success, None on overflow
    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
        //todo!()
        let result = OperationType::Multiplication.perform(x, y);
        let op = Operation::new(x, y, OperationType::Multiplication);
        self.history.push(op);
        result
    }
    
    // TODO: Generate a formatted string showing all operations in history
    // Format: "index: first_num operation_sign second_num = result\n"
    //
    // Example: "0: 5 + 3 = 8\n1: 10 - 2 = 8\n"
    pub fn show_history(&self) -> String{
        //todo!()
        let mut history_str = String::new();
        for (i, op) in self.history.iter().enumerate() {
            let sign = op.operation_type.get_sign();
            // Re-calculate the result for display, as Operation struct doesn't store it
            let result = op.operation_type.perform(op.first_num, op.second_num); 
            match result {
                Some(res) => history_str.push_str(&format!("{}: {} {} {} = {}\n", i, op.first_num, sign, op.second_num, res)),
                None => history_str.push_str(&format!("{}: {} {} {} = Overflow\n", i, op.first_num, sign, op.second_num)),
            }
        }
        history_str
    }
    
    // TODO: Repeat an operation from history by index
    // Add the repeated operation to history and return the result
    // Return None if the index is invalid
    pub fn repeat(&mut self, operation_index: usize) -> Option<i64>{
        //todo!()
        if let Some(op_to_repeat) = self.history.get(operation_index).cloned() {
            // Re-perform the operation to get its result
            let result = op_to_repeat.operation_type.perform(op_to_repeat.first_num, op_to_repeat.second_num);
            
            // Create a new Operation entry for the history (without storing result in struct)
            let new_op = Operation::new(op_to_repeat.first_num, op_to_repeat.second_num, op_to_repeat.operation_type);
            self.history.push(new_op);
            result // Return the result of the repeated operation
        } else {
            None // Return None if the index is out of bounds
        }
    }
    
    // TODO: Clear all operations from history
    pub fn clear_history(&mut self) {
        //todo!()
        self.history.clear();
    }
}
