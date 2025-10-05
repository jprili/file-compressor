use std::fmt::{ Debug };

type TreePtr<T> = Option<Box<Tree<T>>>;

pub struct Tree<T: Debug> {
    pub data: T,
    pub left: TreePtr<T>,
    pub right: TreePtr<T>
}

impl<T: Debug> Tree<T> {
    pub fn new(data: T) -> Self {
        Self { 
            data: data, 
            left: None,
            right: None 
        }
    }

    pub fn set_data(&mut self, value: T) {
        self.data = value;
    }

    pub fn insert(&mut self, left: T, right: T) {
        self.left  = Some(Box::new(Tree::new(left)));
        self.right = Some(Box::new(Tree::new(right)));
    }

    pub fn remove(&mut self, left: bool, right: bool) {
        if left {
            self.left = None;
        }
        if right {
            self.right = None;
        }
    }

    fn has_children(self) -> bool {
        !(self.left.is_none() || self.right.is_none())
    }

    pub fn print(self) {
        match self.left {
            Some(tree) => tree.print(), 
            None => println!("")
        }
        println!("{:?}", self.data);
        match self.right {
            Some(tree) => tree.print(), 
            None => println!("")
        }
    }
}
