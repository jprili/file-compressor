pub struct Node <'a>{
    data:  Option<u128>,
    left:  Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>
}

impl Node<'_>{
    pub fn new(data: Option<u128>) -> Self {
        Self {
            data,
            left: None,
            right: None
        }
    }
}
