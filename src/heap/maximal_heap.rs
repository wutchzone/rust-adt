pub struct MaximalHeap {
    data: Vec<i32>,
}

impl MaximalHeap {
    pub fn new() -> MaximalHeap {
        MaximalHeap { data: Vec::new() }
    }

    pub fn insert(&mut self, d: i32) {
        self.data.push(d);
        self.bubble_up(self.data.len() - 1);
    }

    pub fn extact_max(&mut self) -> Option<i32> {
        if self.data.len() == 0 {
            return None;
        };

        let res = self.data[0];
        if self.data.len() != 1 {
            self.data[0] = self.data.pop().unwrap();
            self.bubble_down(0);
        } else {
            self.data.pop();
        }

        Some(res)
    }

    fn bubble_down(&mut self, index: usize) {
        let current = self.data[index];
        let left = if index * 2 + 1 < self.data.len() {
            Some(self.data[index * 2 + 1])
        } else {
            None
        };
        let right = if index * 2 + 2 < self.data.len() {
            Some(self.data[index * 2 + 2])
        } else {
            None
        };

        if left == None {
            return;
        } else if right == None {
            if left.unwrap() > current {
                self.data[index * 2 + 1] = self.data[index];
                self.data[index] = left.unwrap();
            }
        } else if left.unwrap() >= right.unwrap() {
            if current < left.unwrap() {
                self.data[index * 2 + 1] = current;
                self.data[index] = left.unwrap();
                self.bubble_down(index * 2 + 1); // bubble to child
            }
        } else {
            if current < right.unwrap() {
                self.data[index * 2 + 2] = current;
                self.data[index] = right.unwrap();
                self.bubble_down(index * 2 + 2); // bubble to child
            }
        }
    }

    fn bubble_up(&mut self, index: usize) {
        let parent = self.data[index / 2];
        let left = if (index / 2) * 2 + 1 < self.data.len() {
            Some(self.data[(index / 2) * 2 + 1])
        } else {
            None
        };
        let right = if (index / 2) * 2 + 2 < self.data.len() {
            Some(self.data[(index / 2) * 2 + 2])
        } else {
            None
        };

        if right != None {
            let left = left.unwrap();
            let right = right.unwrap();
            if left >= parent && left > right {
                self.data[index / 2] = left;
                self.data[(index / 2) * 2 + 1] = parent;
            } else if right > parent && left < right {
                self.data[index / 2] = right;
                self.data[(index / 2) * 2 + 2] = parent;
            }
        } else if left != None {
            let left = left.unwrap();
            if left >= parent {
                self.data[index / 2] = left;
                self.data[(index / 2) * 2 + 1] = parent;
            }
        }

        if index != 0 {
            self.bubble_up(index / 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_inserts() {
        let mut h = MaximalHeap::new();
        assert_eq!(h.extact_max(), None);
        h.insert(1);
        h.insert(2);
        h.insert(3);

        assert_eq!(h.data, vec![3, 1, 2]);
        assert_eq!(h.extact_max(), Some(3));
        assert_eq!(h.data, vec![2, 1]);
        assert_eq!(h.extact_max(), Some(2));
        assert_eq!(h.data, vec![1]);
        assert_eq!(h.extact_max(), Some(1));
        assert_eq!(h.extact_max(), None);
    }
    #[test]
    fn advanced_inserts() {
        let mut h = MaximalHeap::new();
        assert_eq!(h.extact_max(), None);
        h.insert(6);
        h.insert(1);
        assert_eq!(h.data, vec![6, 1]);
        h.insert(5);
        assert_eq!(h.data, vec![6, 1, 5]);
        h.insert(3);
        assert_eq!(h.data, vec![6, 3, 5, 1]);
        h.insert(10);
        assert_eq!(h.data, vec![10, 6, 5, 1, 3]);
        h.insert(4);
        assert_eq!(h.data, vec![10, 6, 5, 1, 3, 4]);
        h.insert(2);
        assert_eq!(h.data, vec![10, 6, 5, 1, 3, 4, 2]);
        h.insert(6);
        assert_eq!(h.data, vec![10, 6, 5, 6, 3, 4, 2, 1]);

        assert_eq!(h.extact_max(), Some(10));
        assert_eq!(h.extact_max(), Some(6));
        assert_eq!(h.extact_max(), Some(6));
        assert_eq!(h.extact_max(), Some(5));
        assert_eq!(h.extact_max(), Some(4));
    }
    #[test]
    fn build_heap() {
        assert_eq!(3 + 2, 5);
    }
}
