use std::any::Any;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

impl Object for f32 {
    fn type_name(&self) -> &str {"f32"}
    fn as_any(&self) -> &dyn Any {self}
}

pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}

fn integrate(operations: Vec<String>) -> Vec<String> {

    let mut operations_integrateds: Vec<String> = Vec::new();

    for operation in 0..operations.len() {
        
        if operations[operation].contains("x") {
            
            if operations[operation] == "-x" {
                operations_integrateds.push(("-x^2/2").to_string());
            }

            if operations[operation] == "x" {
                operations_integrateds.push(("x^2/2").to_string());
            }
            
            if is_of_type::<f32>(&operations[operation].parse::<f32>().unwrap()) {
                operations_integrateds.push((format!("{}x", operations[operation])).to_string());
            }

        }
    
    }

    operations
}

pub(crate) fn calculate(operations: Vec<String>, _limits_operations: Vec<Vec<f32>>) {

    let _operations_integrated: Vec<String> = integrate(operations);

}