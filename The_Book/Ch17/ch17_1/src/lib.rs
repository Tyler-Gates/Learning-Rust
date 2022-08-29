pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
//OOPD - Inheritance.. not exactly, but we have bounded parametric polymorphism, with trait and type bounds
//     - Encapsulation.. private/public values and functions
//     - Objects.. structs and enums with their implementation of related functions


impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        //encapsulation .. everything is private by default, need pub
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
