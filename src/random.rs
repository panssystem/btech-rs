use rand::Rng;

/// Roll 2 6 sided dice. This is the usual roll for the game.
/// ```
/// use btech_rs::random::roll_2d6;
/// 
/// let mut max: i8 = 1;
/// let mut min: i8 = 13;
/// for i in [0..100] {
///   let x = roll_2d6();
///   if x> max { max = x;}
///   if x<min { min = x}
/// }
/// assert!(min < 13 && min > 1);
/// assert!(max < 13 && max > 1);
/// ```
pub fn roll_2d6() -> i8 {
    roll_d6() + roll_d6()
}

pub fn roll_d6() -> i8 {
    rand::thread_rng().gen_range(1..6)
}