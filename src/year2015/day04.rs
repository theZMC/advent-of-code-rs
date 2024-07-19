use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 4)?;

    let mut five_key = 0;
    let mut six_key = 0;

    let five_compare = 0x00000fff;
    let six_compare = 0x000000ff;

    for i in 0..u32::MAX {
        let data = format!("{}{}", challenge.trim(), i);
        let digest = md5::compute(data.as_bytes());

        let check: u32 = u32::from_be_bytes(digest[..4].try_into().unwrap());

        if check < five_compare {
            if five_key == 0 {
                five_key = i;
            }
            if check < six_compare {
                six_key = i;
            }
        }

        if five_key != 0 && six_key != 0 {
            break;
        }
    }

    Ok((five_key.to_string(), six_key.to_string()))
}
