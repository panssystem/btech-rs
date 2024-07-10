use rand::Rng;

pub trait DiceRoller {
/// Roll 2 6 sided dice. This is the usual roll for the game.
/// ```
/// use btech_rs::random::{DefaultRoller,DiceRoller};
/// 
/// let mut max: i8 = 1;
/// let mut min: i8 = 13;
/// let roller = DefaultRoller{};
/// for i in [0..100] {
///   let x = roller.roll_2d6();
///   if x> max { max = x;}
///   if x<min { min = x}
/// }
/// assert!(min < 13 && min > 1);
/// assert!(max < 13 && max > 1);
/// ```
    fn roll_2d6(&self) -> i8 {
        self.roll_d6() + self.roll_d6()
    }

/// Roll 1 6 sided dice. This is the less common roll for the game.
/// ```
/// use btech_rs::random::{DefaultRoller,DiceRoller};
/// 
/// let mut max: i8 = 1;
/// let mut min: i8 = 13;
/// let roller = DefaultRoller{};
/// for i in [0..100] {
///   let x = roller.roll_d6();
///   if x> max { max = x;}
///   if x<min { min = x}
/// }
/// assert!(min < 7 && min > 0);
/// assert!(max < 7 && max > 0);
/// ```
    fn roll_d6(&self) -> i8 {
        rand::thread_rng().gen_range(1..6)
    }
}

pub struct DefaultRoller;
impl DiceRoller for DefaultRoller{}
