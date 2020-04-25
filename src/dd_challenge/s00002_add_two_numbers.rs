
use std::option::Option;

use super::*;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        Solution::add(l1, l2, 0, ListNode::new(-1))
    }

    fn add(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>,
           mut value: i32, mut sum: ListNode) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && value == 0 {
            return None;
        }

        if let Some(l) = l1 {
            value += l.val;
            l1 = l.next;
        }

        if let Some(r) = l2 {
            value += r.val;
            l2 = r.next;
        }

        sum.val = if value > 9 {
            value - 10
        } else { value };

        sum.next = Solution::add(l1, l2, value / 10, ListNode::new(-1));
        println!("{:?}", sum);
        Some(Box::new(sum))
    }

}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

pub fn do_sth() {
    let mut  node1 = ListNode::new(2);

    let mut n2 = ListNode::new(4);

    let n3 = ListNode::new(3);
    n2.next = Some(Box::new(n3));
    node1.next = Some(Box::new(n2));

    println!("{:?}", node1);
    //println!("{:?}", Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(n2))));

}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_add_two_numbers() {
        use super::ListNode;
        let mut  node1 = ListNode::new(2);

        let mut n2 = ListNode::new(4);

        let n3 = ListNode::new(3);
        n2.next = Some(Box::new(n3));
        node1.next = Some(Box::new(n2));

        let mut node2 = ListNode::new(5);
        let mut n2 = ListNode::new(6);

        let n3 = ListNode::new(4);
        n2.next = Some(Box::new(n3));
        node2.next = Some(Box::new(n2));

        let mut result = ListNode::new(7);
        let mut r2 = ListNode::new(0);
        let r3 = ListNode::new(8);
        r2.next = Some(Box::new(r3));
        result.next = Some(Box::new(r2));

        let res = Solution::add_two_numbers(
            Some(Box::new(node1)), Some(Box::new(node2)));

        let exp: Option<Box<ListNode>> = Some(Box::new(result));
        assert_eq!(res, exp);
    }
}