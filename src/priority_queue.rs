extern crate core;

pub struct PriorityQueue<'a, T> {
    ordering: Box<dyn Fn (&T, &T) -> bool + 'a>,
    items: Vec<T>
}

impl<'a, T> PriorityQueue<'a, T> {
    pub fn len(& self) -> usize{
        self.items.len()
    }
    pub fn new_empty(ordering: impl Fn (&T, &T) -> bool + 'a) -> Self{
        return PriorityQueue{
            ordering: Box::new(ordering),
            items: vec![]
        }
    }
    pub fn new_from_vec(ordering: impl Fn (&T, &T) -> bool + 'a, items: Vec<T>) -> Self{
        let mut q = PriorityQueue{
            ordering: Box::new(ordering),
            items
        };
        for i in (0 .. q.len()).rev(){
            q.bubble_down(i)
        }
        q
    }
    fn parent(& self, index: usize) -> Option<usize>{
        if index == 0 {
            None
        } else {
            Some((index + 1) / 2 - 1)
        }
    }
    fn left_child(& self, index: usize) -> Option<usize>{
        let result = (index + 1) * 2 - 1;
        if result >= self.len(){
            None
        } else {
            Some(result)
        }
    }
    fn right_child(& self, index: usize) -> Option<usize>{
        let result = (index + 1) * 2;
        if result >= self.len(){
            None
        } else {
            Some(result)
        }
    }
    fn bubble_up(& mut self, index: usize){
        match self.parent(index) {
            None => {return;}
            Some(parent_index) => {
                if (self.ordering)(&self.items[index], &self.items[parent_index]){
                    self.items.swap(index, parent_index);
                    self.bubble_up(parent_index)
                }
            }
        }
    }
    fn bigger_value(& self, index1: usize, index2: usize) -> usize{
        if (self.ordering)(&self.items[index1], &self.items[index2]){
            index1
        } else {
            index2
        }
    }
    fn bubble_down(& mut self, index: usize){
        let mut new_index = index;
        if let Some(left) = self.left_child(index){
            new_index = self.bigger_value(left, new_index)
        }
        if let Some(right) = self.right_child(index){
            new_index = self.bigger_value(right, new_index)
        }
        self.items.swap(index, new_index)
    }
    pub fn add_item(& mut self, item: T){
        self.items.push(item);
        self.bubble_up(self.items.len() - 1)
    }
    pub fn find_max(& self) -> Option<&T>{
        self.items.get(0)
    }
    pub fn extract_max(& mut self) -> Option<T>{
        let len = self.len();
        if self.items.is_empty(){
            None
        } else if self.items.len() == 1 {
            self.items.pop()
        } else {
            self.items.swap(0, len - 1);
            let result = self.items.pop();
            self.bubble_down(0);
            result
        }
    }
    pub fn sorted(mut self) -> Vec<T>{
        let mut result = vec![];
        while !self.items.is_empty() {
            result.push(self.extract_max().unwrap())
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_empty() {
        let mut new_tree = PriorityQueue::new_empty(|x: & i64, y| x > y);
        new_tree.add_item(9);
        new_tree.add_item(10);
        new_tree.add_item(15);
        new_tree.add_item(7);
        assert_eq!(new_tree.extract_max().unwrap(), 15);
        assert_eq!(new_tree.find_max().unwrap(), &10);
        assert_eq!(new_tree.extract_max().unwrap(), 10);
        assert_eq!(new_tree.extract_max().unwrap(), 9);
        assert_eq!(new_tree.extract_max().unwrap(), 7);
        assert_eq!(new_tree.extract_max(), None)
    }
    #[test]
    fn test_new_from_vec(){
        let vec = vec![9, 10, 15, 7];
        let mut new_tree = PriorityQueue::new_from_vec(|x: &i64, y| x > y, vec);
        assert_eq!(new_tree.extract_max().unwrap(), 15);
        assert_eq!(new_tree.find_max().unwrap(), &10);
        assert_eq!(new_tree.extract_max().unwrap(), 10);
        assert_eq!(new_tree.extract_max().unwrap(), 9);
        assert_eq!(new_tree.extract_max().unwrap(), 7);
        assert_eq!(new_tree.extract_max(), None);
    }
    #[test]
    fn test_sorted(){
        let vec = vec![9, 10, 15, 7];
        let new_tree = PriorityQueue::new_from_vec(|x: &i64, y| x > y, vec);
        assert_eq!(new_tree.sorted(), vec![15, 10, 9, 7])
    }
}
