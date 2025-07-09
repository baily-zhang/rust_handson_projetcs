pub fn greed_coin_change(mut amount: u32) -> Vec<u32> {
    let mut change = Vec::new();

    while amount > 0 {
        match amount {
            n if n >= 25 => {
                amount -= 25;
                change.push(25);
            }
            n if n >= 10 => {
                amount -= 10;
                change.push(10);
            }
            n if n >= 5 => {
                amount -= 5;
                change.push(5);
            }
            _ => {
                amount -= 1;
                change.push(1);
            }
        }
    }
    change
}
