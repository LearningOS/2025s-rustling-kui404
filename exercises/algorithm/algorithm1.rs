/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where
    T:PartialOrd+Copy
	{
		// 创建一个新的链表用于存储合并结果
        let mut merged_list=LinkedList::new();
        // 定义两个指针，分别指向两个链表的起始节点
        let mut ptr_a=list_a.start.clone();
        let mut ptr_b=list_b.start.clone();
        // 遍历两个链表，按顺序合并点
        while let (Some(node_a),Some(node_b)) =(&ptr_a,&ptr_b)  {
            let val_a =unsafe {&node_a.as_ref().val};
            let val_b =unsafe{&node_b.as_ref().val};
            if val_a <= val_b {
                // 将 ptr_a 指向的节点添加到合并链表中
                let next_a = unsafe { node_a.as_ref().next };
                merged_list.add_node(ptr_a.unwrap());
                ptr_a = next_a;
            } else {
                // 将 ptr_b 指向的节点添加到合并链表中
                let next_b = unsafe { node_b.as_ref().next };
                merged_list.add_node(ptr_b.unwrap());
                ptr_b = next_b;
            }
        }

        // 如果其中一个链表还有剩余节点，直接将其添加到合并链表中
        while let Some(node_ptr) = ptr_a.take() {
            let next_a = unsafe { node_ptr.as_ref().next };
            merged_list.add_node(node_ptr);
            ptr_a = next_a;
        }

        while let Some(node_ptr) = ptr_b.take() {
            let next_b = unsafe { node_ptr.as_ref().next };
            merged_list.add_node(node_ptr);
            ptr_b = next_b;
        }

        // 更新合并链表的长度
        merged_list.length = list_a.length + list_b.length;

        merged_list
    }

    /// 添加一个节点到链表末尾
    fn add_node(&mut self, node_ptr: NonNull<Node<T>>) {
        match self.end {
            None => {
                self.start = Some(node_ptr);
                self.end = Some(node_ptr);
            }
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = Some(node_ptr);
                self.end = Some(node_ptr);
            },
        }
    }
        }
// 		Self {
//             length: 0,
//             start: None,
//             end: None,
//         }
// 	}
// }

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}