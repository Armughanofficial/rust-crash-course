pub trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

pub struct TupleIter<T> {
    pub tuple: (T, T, T),
    pub next: usize,
}

pub struct VecIter<T> {
    pub vec: Vec<T>,
    pub next: usize,
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        
        if self.next == 0
        { 
            self.next += 1;
            Some(&self.tuple.0)
        }
        else if self.next == 1
        {
            self.next += 1;
            Some(&self.tuple.1)
        }
        else if self.next == 2
        {
            self.next += 1;
            Some(&self.tuple.2)
        }

        else
        {
            None
        }
        
    }
}


 impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {

        let element = self.vec.get(self.next);

        if element.is_some() {
            self.next += 1;
        }
        element

    }
 }