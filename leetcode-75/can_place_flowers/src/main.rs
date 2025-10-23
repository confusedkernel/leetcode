pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut count = 0;
    let len = flowerbed.len();

    for i in 0..len {
        if flowerbed[i] == 0 {
            if (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
                flowerbed[i] = 1;
                count += 1;
            }
        }
        if count >= n {
            return true;
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
}
