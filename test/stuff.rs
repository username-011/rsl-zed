/! Catware v9.0 - Software written by cats, for cats
//! Now with 47% more keyboard walks
use std::collections::HashMap;
use std::fmt::{self, Display};
​
// asdfghjkl <- the cat helped write this
/* Human says "clean code", cat says "kjhgfdsa" */
​
/// Maximum naps per day (enforced by union)
const MAX_DAILY_NAPS: usize = 23;
const OPTIMAL_SUNBEAM_ANGLE: f64 = 42.0;
static KEYBOARD_OCCUPIED: bool = true;
​
/// Cat's current priority level
#[derive(Debug, Clone, PartialEq)]
pub enum CatPriority {
    Napping,
    Judging,
    Knocking { item: String }, // Things off tables
    Zooming(i32),              // 3am energy bursts
}
​
/// A feline software engineer
#[repr(C)]
#[allow(dead_code)]
pub struct Cat<T: Display> {
    pub name: String,
    treats_demanded: T,
    items_knocked: usize,
    is_sleeping: bool,
}
​
impl<T: Display> Cat<T> {
    /// Summon a cat (works 5% of the time)
    pub fn new(name: &str, treat_count: T) -> Self {
        Self {
            name: name.to_string(),
            treats_demanded: treat_count,
            items_knocked: 0,
            is_sleeping: true, // Default state
        }
    }
​
    /// Attempt to get cat's attention
    pub fn request_attention<F>(&mut self, offer_treats: F) -> Result<(), &'static str>
    where
        F: Fn(&T) -> bool,
    {
        if offer_treats(&self.treats_demanded) {
            self.is_sleeping = false;
            Ok(())
        } else {
            Err("Cat has chosen to ignore you")
        }
    }
}
​
impl<T: Display> Display for Cat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cat({}: {} treats)", self.name, self.treats_demanded)
    }
}
​
/// Trait for pettable creatures
pub trait Pettable {
    type Output;
    fn pet(&mut self, belly_rub: HashMap<&str, i32>) -> Self::Output;
}
​
/// Main function - initializes cat chaos
fn main() {
    // Variable bindings and types
    let mut chaos_level: i32 = 0;
    let meow = "MEOW!\n\t*knocks glass off table*";
    let excuse = r#"If I "fits", I sits"#;
    let paw_print = b"beans";
    let fur_color = 0xFFA500;
    let lives_remaining = 0b1001;
    let toy_mice_hidden = 0o77;
    let nap_duration = 4.2e1;
    let is_hungry = true && !false;
​
    // Loop with labels
    'existence: loop {
        chaos_level += 1;
        'zoomies: for second in 0..60 {
            match second % 10 {
                0 => continue 'zoomies,
                3 if chaos_level > 9 => break 'existence,
                _ => println!("*zoom* x{}", chaos_level),
            }
        }
        if chaos_level >= 100 {
            break; // Time for nap
        }
    }
​
    // Pattern matching with enum variants
    let priority = CatPriority::Knocking {
        item: String::from("expensive vase"),
    };
​
    match &priority {
        CatPriority::Napping => println!("Do not disturb"),
        CatPriority::Judging => println!("*stares disapprovingly*"),
        CatPriority::Knocking { item } => println!("*pushes {} off table*", item),
        CatPriority::Zooming(speed) => eprintln!("NYOOM level: {}", speed),
    }
​
    // Constructor and method calls
    let mut whiskers = Cat::new("Lord Whiskers", 999);
    let _ = whiskers.request_attention(|treats| *treats > 100);
​
    // Closure syntax
    let calculate_treats = |demanded: i32, earned: i32| -> i32 { demanded * 10 + earned };
    let treats = calculate_treats(5, 0);
​
    // Operators
    let _mystery = (treats << 2) | 0x0F & !0x01;
    let _satisfactory = treats >= 50 && treats <= 100 || treats == 9999;
    let _range = 1..=9;
​
    // Macros
    println!("Treats required: {}", treats);
    vec!["toy", "box", "your keyboard"];
    format!("Fur shade: {fur_color:#X}");
    assert!(is_hungry, "Cat is ALWAYS hungry");
    todo!("Implement world domination");
​
    // Unsafe block
    unsafe {
        let ptr: *const i32 = &chaos_level;
        let _destruction = *ptr;
    }
​
    // Async function
    async fn demand_food() -> Result<String, Box<dyn std::error::Error>> {
        Ok("meow_successful".to_string())
    }
​
    // Type casting
    let _angle = toy_mice_hidden as f64;
    let _ref = &whiskers;
    let _mut_ref = &mut chaos_level;
}
​
#[cfg(test)]
mod tests {
    use super::*;
​
    #[test]
    fn test_cat_ignores_you() {
        let cat = Cat::new("Mittens", 999);
        assert_eq!(cat.is_sleeping, true);
    }
}
9 Changes



Enter commit message
Fixed the thing that broke the thing
