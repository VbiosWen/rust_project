// use std::collections::HashMap;
//
// pub struct Solution {}
//
// impl Solution {
//     pub fn remove_duplicate_letters(s: String) -> String {
//         let mut map: HashMap<char, i32> = HashMap::new();
//         for item in s.chars() {
//             let count = match map.get(&item) {
//                 Some(value) => *value,
//                 _ => 1,
//             };
//             map.insert(item, count);
//         }
//         let mut queue = Vec::new();
//         for item in s.chars() {
//             for (index, value) in queue.iter().enumerate() {
//                 let mut count = match map.get(value) {
//                     Some(value) => value,
//                     _ => &0,
//                 };
//                 if *value > item && *count > 1 {
//                     queue.remove(index);
//                 }
//                 queue.push(item);
//             }
//         }
//         let mut value = String::new();
//         for item in queue.iter() {
//             value.push(*item);
//         }
//         value
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn remove_duplicate_letters() {
//         let letters = Solution::remove_duplicate_letters(String::from("test"));
//         println!("{}", letters);
//     }
// }
