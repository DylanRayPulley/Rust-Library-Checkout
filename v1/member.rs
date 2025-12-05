pub struct Member {
    name: String,
    borrowed: Vec<String>,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            borrowed: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn borrow(&mut self, id: &str) -> Result<(), String> {
        if self.borrowed.len() >= 5 {
            return Err("Cannot borrow more than 5 items".to_string());
        }

        if self.borrowed.iter().any(|found| found == id) {
            return Err("Item already borrowed".to_string());
        }

        self.borrowed.push(id.to_string());
        Ok(())
    }


    pub fn return_item(&mut self, id: &str) -> Result<(), String> {
        if let Some(pos) = self.borrowed.iter().position(|found| found == id) {
            self.borrowed.remove(pos);
            Ok(())
        } else {
            return Err("Item no currently borrowed".to_string())
        }
    }

    pub fn borrowed_ids(&self) -> Vec<String> {
        self.borrowed.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_more_than_five(){
        let mut new_member = Member::new("Clark");

        for i in 0..5 {
            let id = format!("B{}", i);
            assert!(m.borrow(&id).is_ok());
        }

        let result = new_member.borrow("B5");
        assert!(result.is_err());

    }

    #[test]
    fn return_borrowed_item() {
        let new_member = Member::new("Clark");

        let result = new_member.return_item("B1");
        assert!(result.is_err());
    }
}