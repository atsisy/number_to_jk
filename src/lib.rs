const BASIC_UNIT: [char; 3] = ['十', '百', '千'];
const LARGE_UNIT: [char; 5] = [ '万', '億', '兆', '京', '垓'];

fn large_unit_digits(digits: u64) -> bool {
    (digits % 4) == 0
}

fn unit_number_to_jk(un: char) -> char {
    match un {
	'1' => '一',
	'2' => '二',
	'3' => '三',
	'4' => '四',
	'5' => '五',
	'6' => '六',
	'7' => '七',
	'8' => '八',
	'9' => '九',
	_ => panic!("Error: {}", un),
    }
}

fn __number_to_jk(num_str: &str) -> String {
	let mut unit = 0;
	let mut large_unit_flag = false;
    let mut ret_str: String = "".to_string();
    
    for number in num_str.chars().rev() {
	// numberが'0'なら無視
	if number != '0' {
		print!("{}", number);
	    if large_unit_digits(unit) {
			large_unit_flag = false;
			// 万, 億などの単位が入る場合
			if unit / 4 > 0 {
				large_unit_flag = true;
		    	ret_str.push(LARGE_UNIT[((unit / 4) - 1) as usize]);
			}
			if number == '1' {
		    	// '1'に関しては漢数字の'一'を挿入する
		    	ret_str.push('一');
			}
	    } else {
			// 万, 億などの単位が入らない場合
			if !large_unit_flag && unit / 4 > 0 {
				large_unit_flag = true;
				ret_str.push(LARGE_UNIT[((unit / 4) - 1) as usize]);
			}

			ret_str.push(BASIC_UNIT[((unit % 4) - 1) as usize]);
	    }

	    if number != '1' {
		let ch = unit_number_to_jk(number);
		ret_str.push(ch);
	    }
	}
	
	unit += 1;
    }

    ret_str.chars().rev().collect::<String>()
}

pub fn number_to_jk(value: u64) -> String {
    let num_str: String = format!("{}", value);
    if num_str == "0" {
	"○".to_string()
    } else {
	__number_to_jk(&num_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::number_to_jk;
    #[test]
    fn it_works() {
	assert_eq!(number_to_jk(100000), "十万");
	assert_eq!(number_to_jk(1100000), "百十万");
	assert_eq!(number_to_jk(11100000), "千百十万");
	assert_eq!(number_to_jk(1100), "千百");
	assert_eq!(number_to_jk(239432), "二十三万九千四百三十二");
	assert_eq!(number_to_jk(1230981329), "十二億三千九十八万千三百二十九");
	assert_eq!(number_to_jk(1110), "千百十");
	assert_eq!(number_to_jk(2110), "二千百十");
	assert_eq!(number_to_jk(12), "十二");
	assert_eq!(number_to_jk(11), "十一");
	assert_eq!(number_to_jk(10), "十");
	assert_eq!(number_to_jk(9), "九");
	assert_eq!(number_to_jk(8), "八");
	assert_eq!(number_to_jk(1), "一");
	assert_eq!(number_to_jk(0), "○");
	assert_eq!(number_to_jk(22), "二十二");
	assert_eq!(number_to_jk(52), "五十二");
	assert_eq!(number_to_jk(252), "二百五十二");
	assert_eq!(number_to_jk(20000), "二万");
	assert_eq!(number_to_jk(200000000), "二億");
	assert_eq!(number_to_jk(18446744073709551615), "千八百四十四京六千七百四十四兆七百三十七億九百五十五万千六百十五")
    }
}
