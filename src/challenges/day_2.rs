#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Spreadsheet {
    data: Vec<Vec<u32>>,
}

fn min_max<T: Ord>(slice: &[T]) -> Option<(&T, &T)> {
    Some((slice.iter().min()?, slice.iter().max()?))
}

fn find_divisible(slice: &[u32]) -> Option<u32> {
    for (i, &num) in slice.iter().enumerate() {
        for (j, &denom) in slice.iter().enumerate() {
            if num % denom == 0 && i != j {
                return Some(num / denom);
            }
        }
    }

    None
}

impl Spreadsheet {
    fn checksum(&self) -> u32 {
        self.data.iter().map(|vec| match min_max(vec) {
            Some((lo, hi)) => hi - lo,
            None => 0
        }).sum()
    }

    fn sum_divisible(&self) -> u32 {
        self.data.iter()
            .map(|vec| find_divisible(vec).unwrap_or(0))
            .sum()
    }
}

impl ::std::str::FromStr for Spreadsheet {
    type Err = Box<::std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for row in s.lines() {
            let row = row.split_whitespace().map(|item| item.parse()).collect::<Result<Vec<_>, _>>()?;
            data.push(row);
        }

        Ok(Spreadsheet { data })
    }
}

pub fn puzzle() {
    let sheet = ::read_file(::first_arg()).parse::<Spreadsheet>().expect("spreadsheet parse fail");
    println!("Part 1: {}", sheet.checksum());
    println!("Part 2: {}", sheet.sum_divisible());
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;

    #[test]
    fn test_spreadsheet_parse() {
        let input = "1\t2\t3\t4\n5\t6\t7\n9\t10\t11\t12";
        assert_eq!(input.parse::<Spreadsheet>().expect("parse fail"), Spreadsheet { data: vec![
            vec![1, 2,  3,  4],
            vec![5, 6,  7],
            vec![9, 10, 11, 12],
        ] });
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[12, 52, 77, 35]), Some((&12, &77)));
        assert_eq!(min_max(&[53, 63, 19]),     Some((&19, &63)));
        assert_eq!(min_max(&[19, 62, 13, 84]), Some((&13, &84)));
    }

    #[test]
    fn test_checksum() {
        let sheet = Spreadsheet { data: vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8],
        ] };

        assert_eq!(sheet.checksum(), 18);
    }

    #[test]
    fn test_divisible() {
        assert_eq!(find_divisible(&[5, 9, 2, 8]), Some(4));
        assert_eq!(find_divisible(&[9, 4, 7, 3]), Some(3));
        assert_eq!(find_divisible(&[3, 8, 6, 5]), Some(2));
        assert_eq!(find_divisible(&[2, 3, 5, 7]), None);
    }
}
