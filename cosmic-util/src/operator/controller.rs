use anchor_lang::solana_program::pubkey::Pubkey;

pub fn can_execute(operators: &Vec<Pubkey>, stream_key: &Pubkey, current_operator: &Pubkey) -> bool {
    let stream_id = stream_key.to_bytes()[0] as usize;

    let n = operators.len();
    let operator_index = stream_id % n;
    let operator_in_charge = operators.get(operator_index).unwrap();

    operator_in_charge == current_operator
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::__private::bytemuck::__core::str::FromStr;

    #[test]
    fn test_program_settings() {
        let mut operators: Vec<Pubkey> = Vec::new();

        let operator0 = Pubkey::from_str("EpmRY1vzTajbur4hkipMi3MbvjbJHKzqEAAqXj12ccZQ").unwrap();
        operators.push(operator0);

        let operator1 = Pubkey::from_str("86G3gad5tVjJxdQmmdQ6E3rLQNnDNh4KYcqiiSd7Az63").unwrap();
        operators.push(operator1);

        let operator2 = Pubkey::from_str("AbugGcRTG2rhAqvE6U4t5qH1ftedcKgEa19BjHbFGCMG").unwrap();
        operators.push(operator2);

        let stream1 = Pubkey::from_str("9dt6a11nz8EXg7HBo7tqcSqguwBAUDoHvR7nGZPvuu6X").unwrap();
        let checkop0 = can_execute(&operators, &stream1, &operator0);
        let checkop1 = can_execute(&operators, &stream1, &operator1);
        let checkop2 = can_execute(&operators, &stream1, &operator2);
        println!(
            "Op 1 - {}, Op 2 - {}, Op 3 - {}",
            checkop0, checkop1, checkop2
        );

        let stream2 = Pubkey::from_str("76eTpjuD3EUHthbHKqLzXWFRmDTgEcZYPK4hCWVFJvYk").unwrap();
        let checkop0 = can_execute(&operators, &stream2, &operator0);
        let checkop1 = can_execute(&operators, &stream2, &operator1);
        let checkop2 = can_execute(&operators, &stream2, &operator2);
        println!(
            "Op 1 - {}, Op 2 - {}, Op 3 - {}",
            checkop0, checkop1, checkop2
        );

        let stream3 = Pubkey::from_str("Bbfi7ztGB6NfaDNiW6ietjpPZr3MgsxeZDrA5mMJKHDZ").unwrap();
        let checkop0 = can_execute(&operators, &stream3, &operator0);
        let checkop1 = can_execute(&operators, &stream3, &operator1);
        let checkop2 = can_execute(&operators, &stream3, &operator2);
        println!(
            "Op 1 - {}, Op 2 - {}, Op 3 - {}",
            checkop0, checkop1, checkop2
        );
    }
}
