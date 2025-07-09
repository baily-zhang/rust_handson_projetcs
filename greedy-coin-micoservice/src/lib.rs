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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_amount() {
        let result = greed_coin_change(0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_penny() {
        let result = greed_coin_change(1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_nickel() {
        let result = greed_coin_change(5);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_dime() {
        let result = greed_coin_change(10);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_quarter() {
        let result = greed_coin_change(25);
        assert_eq!(result, vec![25]);
    }

    #[test]
    fn test_mixed_coins() {
        let result = greed_coin_change(67);
        // 67 = 25 + 25 + 10 + 5 + 1 + 1
        assert_eq!(result, vec![25, 25, 10, 5, 1, 1]);
    }

    #[test]
    fn test_large_amount() {
        let result = greed_coin_change(99);
        // 99 = 25 + 25 + 25 + 10 + 10 + 1 + 1 + 1 + 1
        assert_eq!(result, vec![25, 25, 25, 10, 10, 1, 1, 1, 1]);
    }

    #[test]
    fn test_exact_denominations() {
        let result = greed_coin_change(41);
        // 41 = 25 + 10 + 5 + 1
        assert_eq!(result, vec![25, 10, 5, 1]);
    }
}
