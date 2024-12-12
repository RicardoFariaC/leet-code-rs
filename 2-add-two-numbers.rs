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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut curr = &mut head;
        let mut aux = 0;
    
        while l1.is_some() || l2.is_some() || aux != 0 {
            let s1 = l1.as_ref().map_or(0, |x| x.val);
            let s2 = l2.as_ref().map_or(0, |x| x.val);
            let sum = s1 + s2 + aux;

            aux = sum / 10;
            let sum_node = curr.insert(Box::new(ListNode::new(sum % 10))); // &mut Box<ListNode>
            curr = &mut sum_node.next; 
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }
        head
    }
}
