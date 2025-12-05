use super::items::Item;

pub struct Catalog {
    items: Vec<Box<dyn Item>>,
}

impl Catalog {

    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn add(&mut self, item: Box<dyn Item>) -> Result<(), String> {
        let id = item.id().to_string();

        if self.items.iter().any(|obj| obj.id() == id) {
            return Err(format!("Item with id: {} already exists", id));
        }

        self.items.push(item);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<&dyn Item, String> {

        self.items.iter().find(|obj| obj.id() == id).map(|i| i.as_ref()).ok_or_else( || format!("Item id: {} does not exist", id))
    }

    pub fn details_for(&self, ids: &[String]) -> Vec<(String, String, u32)> {
        
        let mut result = Vec::new();

        for i in ids {
            if let Ok(item) = self.get(i) {
                result.push((item.id().to_string(), item.title().to_string(), item.days_allowed()))
            }
        }

        result
    }

}
