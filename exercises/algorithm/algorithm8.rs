/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

enum WhichQueue {
    First,
    Second,
}
pub struct myStack<T>
{
	//TODO
    size: usize,
    cur: WhichQueue,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            size: 0usize,
            cur: WhichQueue::First,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        match self.cur {
            WhichQueue::First => {
                self.q1.enqueue(elem);
            }
            WhichQueue::Second => {
                self.q2.enqueue(elem);
            }
        }
        self.size += 1;
        
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.size == 0 {
            return Err("Stack is empty");
        }
        match self.cur {
            WhichQueue::First => {
                for _i in 1..self.size {
                    self.q2.enqueue(self.q1.dequeue().unwrap());
                }
                self.size -= 1;
                self.cur = WhichQueue::Second;
                Ok(self.q1.dequeue().unwrap())
            }
            WhichQueue::Second => {
                for _i in 1..self.size {
                    self.q1.enqueue(self.q2.dequeue().unwrap());
                }
                self.size -= 1;
                self.cur = WhichQueue::First;
                Ok(self.q2.dequeue().unwrap())
            }
        }
    }
    pub fn is_empty(&self) -> bool {
		self.size == 0
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}