pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let mut avg_coll = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        avg_coll.add(1);
        avg_coll.add(2);
        avg_coll.add(3);
        avg_coll.add(4);
        avg_coll.add(5);

        assert_eq!(avg_coll.average(), 3.0);

        let removed_val = avg_coll.remove();

        assert_eq!(removed_val, Some(5));

        assert_eq!(avg_coll.average(), 2.5);
    }
}
