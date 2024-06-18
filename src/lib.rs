#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8
{
    //Enumeration
    use Category::*;

    //set the dice variable to mutable
    let mut dice = dice;

    //sort
    dice.sort();

    //match
    match (category, dice)
    {

        //Ones, Logic: Any combination, Score: 1 * count
        (Ones, _) => dice.into_iter().filter(|x| *x == 1).sum(),
        
        //Twos, Logic: Any combination, Score: 2 * count
        (Twos, _) => dice.into_iter().filter(|x| *x == 2).sum(),

        //Threes, Logic: Any combination, Score: 3 * count
        (Threes, _) => dice.into_iter().filter(|x| *x == 3).sum(),

        //Fours, Logic: Any combination, Score: 4 * count
        (Fours, _) => dice.into_iter().filter(|x| *x == 4).sum(),

        //Fives, Logic: Any combination, Score: 5 * count
        (Fives, _) => dice.into_iter().filter(|x| *x == 5).sum(),

        //Sixes, Logic: Any combination, Score: 6 * count
        (Sixes, _) => dice.into_iter().filter(|x| *x == 6).sum(),

        //Full House, Logic: Three of one number and two of another, Score: Dice total
        (FullHouse, [a, b, c, d, e]) if (
            (a == b) && 
            ((b == c) || (c == d)) && 
            (d == e) && 
            (a != e)
        ) => dice.into_iter().sum(),

        //Four Of A Kind, Logic: At leat four of the same digit, Four dice total
        (FourOfAKind, [a, b, c, d, _] | [_, a, b, c, d]) if (
            (a == b) && 
            (b == c) && 
            (c == d)
        ) => a + b + c + d,
        
        //Little Straight, Logic: 1, 2, 3, 4, 5, Score: 30 Points
		(LittleStraight, [1, 2, 3, 4, 5]) => 30,

        //Big Straight, Logic: 2, 3, 4, 5, 6, Score: 30 points
		(BigStraight, [2, 3, 4, 5, 6]) => 30,

        //Choice, Logic: Any combination, Score: Dice sum
		(Choice, _) => dice.into_iter().sum(),

        //Yacht, Logic: All dice the same digit, Score: 50 points
		(Yacht, [a, b, c, d, e]) if ((a == b) && (b == c) && (c == d) && (d == e)) => 50,

        //If the dice do not satisfy the requirements of a category, the score is zero
        _ => 0,
    }
    
}