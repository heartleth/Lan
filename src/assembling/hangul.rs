pub fn to_phonemes(c :char)->String {
    let firsts = Vec::from_iter("ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars());
    let middles = Vec::from_iter("ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ".chars());
    let lasts = Vec::from_iter("0ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ".chars());
    
    let uni = c as u32;
    if 0xAC00 <= uni && uni <= 0xD7AF {
        let nth = (uni - 0xAC00) as usize;
        let first = nth / 588;
        let middle = (nth % 588) / 28;
        let last = nth % 28;
        if last > 0 {
            return format!("{}{}{}", firsts[first], middles[middle], lasts[last]);
        }
        else {
            return format!("{}{}", firsts[first], middles[middle]);
        }
    }
    return String::from(c);
}

pub fn disassemble<'s>(s :&'s str) -> Vec<char> {
    let t :Vec<String> = s.chars().map(to_phonemes).collect();
    let ret :Vec<char> = t.join("").chars().collect();
    return ret;
}

pub fn assemble<'s>(s :&'s str) -> String {
    let firsts = Vec::from_iter("ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars());
    let middles = Vec::from_iter("ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ".chars());
    let lasts = Vec::from_iter("ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ".chars());
    
    let mut ret = String::new();
    for c in s.chars() {
        if ret.is_empty() {
            ret.push(c);
        }
        else if firsts.contains(&c) || lasts.contains(&c) {
            let last = ret.chars().last().unwrap();
            let lp = lasts.iter().position(|x| x==&c).unwrap() as u32;
            let unilast = last as u32;
            if 0xAC00 <= unilast && unilast <= 0xD7AF {
                if (unilast - 0xAC00) % 28 == 0 {
                    ret.pop();
                    ret.push(char::from_u32(unilast + lp + 1).unwrap());
                }
                else {
                    ret.push(c);
                }
            }
            else {
                ret.push(c);
            }
        }
        else if middles.contains(&c) {
            let last = ret.chars().last().unwrap();
            let unilast = last as u32;
            let mp = middles.iter().position(|x| x==&c).unwrap() as u32;
            if let Some(pos) = firsts.iter().position(|x| x==&last) {
                let lc = ('가' as u32) + (pos as u32) * 588 + mp * 28;
                ret.pop();
                ret.push(char::from_u32(lc).unwrap());
            }
            else if 0xAC00 <= unilast && unilast <= 0xD7AF {
                if (unilast - 0xAC00) % 28 != 0 {
                    let f = firsts.iter().position(|x| x==&lasts[((unilast - 0xAC00) % 28 - 1) as usize]);
                    if let Some(f) = f {
                        ret.pop();
                        ret.push(char::from_u32(unilast - ((unilast - 0xAC00) % 28)).unwrap());
                        ret.push(char::from_u32(('가' as u32) + (f as u32) * 588 + mp * 28).unwrap());
                    }
                    else {
                        ret.push(c);
                    }
                }
                else {
                    ret.push(c);
                }
            }
            else {
                ret.push(c);
            }
        }
        else {
            ret.push(c);
        }
    }
    return ret;
}