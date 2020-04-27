use crate::dd_challenge::Solution;

impl Solution {
    pub fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();

        let mut head = head;
        let mut res = None;
        while let Some(h) = head {
            stack.push(h.val);
            head = h.next;
        }

        let mut is_plus = true;
        while let Some(v) = stack.pop() {

            println!("The stack value is: {:?}", v);
            let v = if is_plus { v + 1 } else { v };
            is_plus = if v > 9 { true } else { false };
            let mut new_node = Box::new(ListNode::new(v % 10));
            new_node.next = res;
            res = Some(new_node);
            println!("{:?}", res);
        }
        res
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
    let mut node2 = ListNode::new(5);
    let mut n2 = ListNode::new(6);

    let n3 = ListNode::new(4);
    n2.next = Some(Box::new(n3));
    node2.next = Some(Box::new(n2));

    println!("{:?}", node2);
    let res = Solution::plus_one(Some(Box::new(node2)));
    println!("{:?}", res);

}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;
    #[test]
    fn test_add_two_numbers_ii() {
        use super::ListNode;

        let mut node2 = ListNode::new(5);
        let mut n2 = ListNode::new(6);

        let n3 = ListNode::new(4);
        n2.next = Some(Box::new(n3));
        node2.next = Some(Box::new(n2));

        let mut result = ListNode::new(5);
        let mut r2 = ListNode::new(6);
        let mut r3 = ListNode::new(5);

        r2.next = Some(Box::new(r3));
        result.next = Some(Box::new(r2));

        let res = Solution::plus_one(Some(Box::new(node2)));

        let exp: Option<Box<ListNode>> = Some(Box::new(result));
        println!("{:?}, {:?}", exp, res);

        assert_eq!(res, exp);
    }
}