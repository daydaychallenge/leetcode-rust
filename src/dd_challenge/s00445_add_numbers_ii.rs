use crate::dd_challenge::Solution;

impl Solution {
    pub fn add_two_numbers_ii(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        let mut l1 = l1;
        while let Some(l) = l1 {
            stack1.push(l.val);
            l1 = l.next;
        }

        let mut l2 = l2;
        while let Some(l) = l2 {
            stack2.push(l.val);
            l2 = l.next;
        }

        let mut res: Option<Box<ListNode>> = None;

        let mut sum = 0;

        while !stack1.is_empty() || !stack2.is_empty() || sum > 0 {
            let (v1, v2) = (stack1.pop().unwrap_or(0), stack2.pop().unwrap_or(0));
            let tmp = v1 + v2 + sum;
            sum = tmp / 10;
            let mut new_node = Box::new(ListNode::new(tmp % 10));
            new_node.next = res;
            res = Some(new_node);
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

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;
    #[test]
    fn test_add_two_numbers_ii() {
        use super::ListNode;
        let mut n1 = ListNode::new(7);
        let mut  node1 = ListNode::new(2);

        let mut n2 = ListNode::new(4);

        let n3 = ListNode::new(3);
        n2.next = Some(Box::new(n3));
        node1.next = Some(Box::new(n2));
        n1.next = Some(Box::new(node1));

        let mut node2 = ListNode::new(5);
        let mut n2 = ListNode::new(6);

        let n3 = ListNode::new(4);
        n2.next = Some(Box::new(n3));
        node2.next = Some(Box::new(n2));

        let mut result = ListNode::new(7);
        let mut r2 = ListNode::new(8);
        let mut r3 = ListNode::new(0);
        let r4 = ListNode::new(7);
        r3.next = Some(Box::new(r4));
        r2.next = Some(Box::new(r3));
        result.next = Some(Box::new(r2));

        let res = Solution::add_two_numbers_ii(
            Some(Box::new(n1)), Some(Box::new(node2)));

        let exp: Option<Box<ListNode>> = Some(Box::new(result));
        assert_eq!(res, exp);
    }
}