use crate::custom_error::AocError;

use itertools::Itertools;

use std::cmp::Ordering;
use std::str::FromStr;

/// In increasing strength order
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Store each hand-bid pair with corresponding category
#[derive(Debug, Eq)]
struct HandBid {
    hand: [u8; 5],
    bid: u32,
    category: Category,
}

impl HandBid {
    const J_VALUE: u32 = 1;
    const T_VALUE: u32 = 10;
    const Q_VALUE: u32 = 11;
    const K_VALUE: u32 = 12;
    const A_VALUE: u32 = 13;
}

/// Calculate and returns category based on hand
fn get_category(hand: &[u8; 5]) -> Category {
    // Store count for each card value. (index + 1) corresponds to card value.
    let mut card_counts = [0_u8; 13];

    // Stores count for each card count. Index corresponds to card count for a particular value.
    let mut card_count_counts = [0_u8; 6];

    let mut position_max = 0;
    let mut max = 0;

    // Find count for each card value found in hand
    for card_value in hand {
        let card_count = &mut card_counts[*card_value as usize - 1];
        *card_count += 1;
        if *card_value != HandBid::J_VALUE as u8 && max < *card_count {
            // Need to find index of max count
            max = *card_count;
            position_max = *card_value as usize - 1;
        }
    }
    
    // position_max == 0 means there were 5 'J's so we shouldn't make any adjustment
    if position_max != 0 {
        card_counts[position_max] += card_counts[HandBid::J_VALUE as usize - 1];
        card_counts[HandBid::J_VALUE as usize - 1] = 0;
    }

    // Find count of each card count
    for count in &card_counts {
        card_count_counts[*count as usize] += 1;
    }

    match card_count_counts {
        [_, _, _, _, _, 1] => Category::FiveOfAKind,
        [_, _, _, _, 1, _] => Category::FourOfAKind,
        [_, _, 1, 1, _, _] => Category::FullHouse,
        [_, _, _, 1, _, _] => Category::ThreeOfAKind,
        [_, _, 2, _, _, _] => Category::TwoPair,
        [_, _, 1, _, _, _] => Category::OnePair,
        _ => Category::HighCard,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandBidError;

// To parse from &str to HandBid
impl FromStr for HandBid {
    type Err = ParseHandBidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get hand and bid as separate &str
        let (hand_str, bid_str) = s
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or(ParseHandBidError)?;

        let mut hand_chars = hand_str.chars();
        let mut hand = [0_u8; 5];

        // Parse each card char into corresponding card value
        for card in &mut hand {
            let card_char = &hand_chars.next().ok_or(ParseHandBidError)?;
            *card = card_char.to_digit(10).map_or_else(
                || match card_char {
                    'J' => Ok(HandBid::J_VALUE),
                    'T' => Ok(HandBid::T_VALUE),
                    'Q' => Ok(HandBid::Q_VALUE),
                    'K' => Ok(HandBid::K_VALUE),
                    'A' => Ok(HandBid::A_VALUE),
                    _ => Err(ParseHandBidError),
                },
                Ok,
            )? as u8;

            // invalid card value
            if *card == 0 {
                return Err(ParseHandBidError);
            }
        }
        // too many cards
        if hand_chars.next().is_some() {
            return Err(ParseHandBidError);
        }

        let bid = bid_str.parse::<u32>().map_err(|_| ParseHandBidError)?;

        Ok(HandBid {
            hand,
            bid,
            category: get_category(&hand),
        })
    }
}

// To sort HandBids
impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Equal => {
                for (a, b) in self.hand.iter().zip(other.hand.iter()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn process(input: &'static str) -> miette::Result<String, AocError> {
    // Parse lines and sort in order of increasing hand strength
    let mut hand_bids = input
        .lines()
        .map(|line| {
            line.parse::<HandBid>()
                .expect("Hand should contain 5 valid cards and bid should be a positive number")
        })
        .collect::<Vec<_>>();
    hand_bids.sort_unstable();

    let result = hand_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, handbid)| {
            acc + handbid.bid as usize * (rank + 1)
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
