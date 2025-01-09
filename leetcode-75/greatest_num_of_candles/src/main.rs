pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let biggest = *candies.iter().max().unwrap();

    candies
        .iter()
        .map(|&candy| candy + extra_candies >= biggest)
        .collect()
}

fn main() {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_kids_with_candies() {
//         assert_eq!()
//     }
// }
