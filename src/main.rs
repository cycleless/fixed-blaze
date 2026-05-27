
use fixed_blaze::gnosis::fixed::Fixed;

fn main() {
    // Формат Q16.16 (32 бита, 16 под дробную часть)
    let a: Fixed<i32, 16> = Fixed::from_bits(1 << 16); // 1.0
    let b: Fixed<i32, 16> = Fixed::from_bits(2 << 16); // 2.0
    
    let mut c = a * b; // 2.0
    assert_eq!(c.to_bits(), 2 << 16);
    
    // Тест Assign-операторов
    c += b; // 4.0
    assert_eq!(c.to_bits(), 4 << 16);
    
    c >>= 1; // 2.0 (побитовый сдвиг = деление на 2)
    assert_eq!(c.to_bits(), 2 << 16);

    // Тест деления на чистых сдвигах с Banker's Rounding
    let num_div: Fixed<i32, 16> = Fixed::from_bits(5 << 16); // 5.0
    let den_div: Fixed<i32, 16> = Fixed::from_bits(2 << 16); // 2.0
    let res_div = num_div / den_div; // 2.5
    assert_eq!(res_div.to_bits(), (2 << 16) + (1 << 15)); // 2.5
    
    // Отрицательные числа, унарный минус и модуль
    let neg_val: Fixed<i32, 16> = -res_div; // -2.5
    assert_eq!(neg_val.is_negative(), true);
    assert_eq!(neg_val.abs(), res_div);
    
    // Взятие остатка (%)
    let rem_res = num_div % den_div; // 5.0 % 2.0 = 1.0
    assert_eq!(rem_res.to_bits(), 1 << 16);
    
    // Округление: floor, ceil, round
    let fraction_val: Fixed<i32, 16> = Fixed::from_bits((2 << 16) + (1 << 15)); // 2.5
    assert_eq!(fraction_val.floor().to_bits(), 2 << 16); // 2.0
    assert_eq!(fraction_val.ceil().to_bits(), 3 << 16); // 3.0
    assert_eq!(fraction_val.round().to_bits(), 3 << 16); // 3.0 (Nearest, Tie-away)
    
    // Округление отрицательных: -2.5 -> floor(-3.0), ceil(-2.0), round(-2.0)
    let neg_frac: Fixed<i32, 16> = -fraction_val;
    assert_eq!(neg_frac.floor().to_bits(), -3 << 16);
    assert_eq!(neg_frac.ceil().to_bits(), -2 << 16);
    assert_eq!(neg_frac.round().to_bits(), -2 << 16);

    println!("All massive mathematical operations evaluated successfully!");
}
