use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_state: u8,
    pub rc_num: u64,
}

impl Version {
    pub fn parse(text: String) -> Version {
        let mut version = Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre_state: 0,
            rc_num: 0,
        };

        if text.contains("-alpha") {
            version.pre_state = 1;
        }
        if text.contains("-beta") {
            version.pre_state = 2;
        }
        if text.contains("-rc-") {
            version.rc_num =
                match sanitize_numeric_string(text.split_once("-rc-").unwrap().1.trim().into())
                    .parse()
                {
                    Ok(v) => v,
                    Err(_) => 0,
                }
        }

        let segments: Vec<&str> = text.split(".").collect();
        for i in 0..segments.len() {
            let s: String = String::from_str(segments.get(i).unwrap())
                .unwrap()
                .chars()
                .filter(|c| c.is_numeric())
                .collect();

            let v = match s.trim().parse::<u64>() {
                Ok(v) => v,
                Err(_) => 0,
            };

            match i {
                0 => {
                    version.major = v;
                }
                1 => version.minor = v,
                2 => version.patch = v,
                _ => {}
            }
        }

        return version;
    }

    pub fn as_text(&self) -> String {
        let mut str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if self.pre_state == 1 {
            str.push_str("-alpha");
        }
        if self.pre_state == 2 {
            str.push_str("-beta");
        }
        if self.rc_num != 0 {
            str.push_str(format!("-rc-{}", self.rc_num).as_str());
        }
        return str;
    }
}

fn sanitize_numeric_string(str: String) -> String {
    return str.chars().filter(|c| c.is_numeric()).collect::<String>();
}
