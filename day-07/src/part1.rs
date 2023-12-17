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

/// Calculate and returns category based on hand
fn get_category(hand: &[u8; 5]) -> Category {
    // Store count for each card value. (index + 2) corresponds to card value.
    let mut card_counts = [0_u8; 13];

    // Stores count for each card count. Index corresponds to card count for a particular value.
    let mut card_count_counts = [0_u8; 6];

    // Find count for each card value found in hand
    for card_value in hand {
        card_counts[*card_value as usize - 2] += 1;
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
                    'T' => Ok(10),
                    'J' => Ok(11),
                    'Q' => Ok(12),
                    'K' => Ok(13),
                    'A' => Ok(14),
                    _ => Err(ParseHandBidError),
                },
                Ok,
            )? as u8;

            // invalid card value
            if *card < 2 {
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
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
