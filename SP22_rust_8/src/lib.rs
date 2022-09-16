/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct LinkedList<T> {
    pub front: Option<Box<Link<T>>>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<T> {
    thing: T,
    next: Option<Box<Link<T>>>,
}

impl<T: Clone> LinkedList<T> {
/// [DO NOT CHANGE THE STRUCT SIGNATURE]

    /// Implement the `LinkedList` functions below

    /// New instance of LinkedList
    /// It should have no front and a length of 0
    pub fn new() -> Self {
        LinkedList {
            front: None,
            length: 0
        }
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        match self.front {
            Some(_) => false,
            None => true
        }
    }

    /// Adds an element to the front of the list
    /// Hint - use the `front` pointer
    pub fn add_front(&mut self, thing: T) {
        self.length += 1;
        if self.front.is_none() {
            self.front = Some(Box::new(Link::new(thing)));
            return;
        }
        let mut new_node = Link::new(thing);
        new_node.next = self.front.clone();
        self.front = Some(Box::new(new_node));
    }

    /// Adds an element to the back of the list
    /// You must interate through the list to find the end
    pub fn add_back(&mut self, thing: T) {
        self.length += 1;

        if self.front.is_none() {
            self.front = Some(Box::new(Link::new(thing)));
            return;
        }

        let mut curr: &mut Option<Box<Link<T>>> = &mut self.front;
        while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
            curr = &mut curr.as_mut().unwrap().next;
        }

        curr.as_mut().unwrap().next = Some(Box::new(Link::new(thing)));
    }

    /// [HELPER FUNCTION - DO NOT CHANGE]
    /// Returns a vector from the given linked list
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut curr = &self.front;        
        while curr.is_some() {
            vec.push(curr.as_ref().unwrap().thing.clone());
            curr = &curr.as_ref().clone().unwrap().next;
        }
        vec
    }

}

impl<T> Link<T> {

    /// Implement the `Link` constructor below

    /// New instance of Link
    /// It should store the thing and None for its next Link
    fn new(thing: T) -> Self {
        Link {
            thing,
            next: None
        }
    }
}
