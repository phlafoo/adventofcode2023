use crate::custom_error::AocError;

// Store label and focal_length as byte slices to avoid unnecessary type conversion
#[derive(Clone, Copy)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: &'a [u8],
}

impl<'a> Lens<'a> {
    pub fn new(label: &'a [u8], focal_length: &'a [u8]) -> Lens<'a> {
        Lens {
            label,
            focal_length,
        }
    }
}

// Custom Debug impl for displaying byte slices as strings
impl<'a> std::fmt::Debug for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lens")
            .field(
                "label",
                &std::str::from_utf8(self.label).unwrap().to_string(),
            )
            .field(
                "focal_length",
                &std::str::from_utf8(self.focal_length).unwrap().to_string(),
            )
            .finish()
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lens_boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| vec![]);

    for step in input.as_bytes().split(|&c| c == b',') {
        let mut step_index = 0;
        let mut box_index = 0;
        loop {
            match step[step_index] {
                // remove lens
                b'-' => {
                    let b = &mut lens_boxes[box_index];
                    let label = &step[0..step_index];

                    if let Some(pos) = b.iter().position(|lens| lens.label == label) {
                        b.remove(pos);
                    }
                    break;
                }
                // update/insert lens
                b'=' => {
                    let b = &mut lens_boxes[box_index];
                    let label = &step[0..step_index];
                    let focal_length = &step[step_index + 1..];

                    match b.iter_mut().find(|lens| lens.label == label) {
                        Some(lens) => lens.focal_length = focal_length,
                        None => b.push(Lens::new(label, focal_length)),
                    }
                    break;
                }
                c => {
                    box_index = (box_index + c as usize) * 17 % 256;
                }
            }
            step_index += 1;
        }
    }

    // Calculate focusing power
    let result = lens_boxes
        .iter()
        .enumerate()
        .map(|(box_index, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| {
                    (box_index + 1)
                        * (lens_index + 1)
                        * std::str::from_utf8(lens.focal_length)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
