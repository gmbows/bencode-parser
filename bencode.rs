fn ben_encode_int(i:i32) -> String {
	let mut pref : String = "i".to_owned();
	let suf : String = "e".to_owned();
	let stre = i.to_string();
	pref.push_str(&stre);
	pref.push_str(&suf);
	return pref;
}

fn ben_decode_int(s : &str) -> i32 {
	let nprefix = s.strip_prefix("i");
	match nprefix { 
		Some(_nprefix) => {},
		None    => return -1,
	}
	let nsuffix = nprefix.unwrap().strip_suffix("e");
	match nsuffix {
		Some(_nsuffix) => {},
		None    => return -1,
	};
	let res = nsuffix.unwrap().parse();
	match res {
		Ok(res) => return res,
		Err(_res) => return -1,
	}
}

fn ben_encode_str(_s : &str) -> String {
	let mut len : String = _s.len().to_string();
	len.push_str(":");
	len.push_str(_s);
	return len;
}

fn indexof(s : &str, tok : char) -> usize {
	for i in 0..s.len() {
		if s.as_bytes()[i] == tok as u8{
			return i;
		}
	}
	return 0;
}

fn indexofs(s : &str, tok : char, start : usize) -> usize {
	for i in start..s.len() {
		if s.as_bytes()[i] == tok as u8{
			return i;
		}
	}
	return 0;
}

fn ben_decode_str(_s : &str) -> String {
	let idx = indexof(_s,':') as usize;
	let _len : usize = _s.len() as usize;
	let S_s : String = _s.to_string();
	let slice = S_s.get(idx+1.._len);
	match slice {
		Some(slice) => return slice.to_string(),
		None => return "".to_string(),
	}
}

fn ben_encode_list(vec : &Vec<String>) -> String {
	let mut benList : String = "".to_string();
	for i in 0..vec.len() {
		if vec[i].as_bytes()[0].is_ascii_digit() {
			benList.push_str(&ben_encode_int(vec[i].parse().unwrap()));
		} else {
			benList.push_str(&ben_encode_str(&vec[i]));
		}
	}
	let mut l : String = "l".to_owned();
	l.push_str(&benList);
	l.push_str(&"e".to_owned());
	return l;
}

fn ben_decode_list(s : String) -> Vec<String> {
	let mut list : Vec<String> = vec![];
	let mut idx : usize = 1;
	while idx < s.len()-1 {
		if s.as_bytes()[idx] == 'i' as u8 {
			// println!("Got integer");
			let slice = s.get(idx+1..indexofs(&s,'e',idx));
			match slice {
				Some(_slice) => {},
				None => {
					println!("(int) Unable to slice string from {0}-{1}",idx,indexofs(&s,'e',idx));
					return list;
				},
			}
			list.push(slice.unwrap().to_string());
			idx = indexofs(&s,'e',idx)+1;
		} else {
			let s_len : String = s.get(idx..indexofs(&s,':',idx)).unwrap().parse().unwrap();
			// let _len : usize = ((s.as_bytes()[idx])-('0' as u8)) as usize;
			let _len : usize = s_len.parse().unwrap();
			// println!("Got string {}",_len);
			let slice = s.get(idx+s_len.len()+1..idx+_len+s_len.len()+1);
			match slice {
				Some(_slice) => {},
				None => {
					println!("(string) Unable to slice string from {0}-{1}",idx+s_len.len()+1,idx+_len);
					return list;
				},
			}
			list.push(slice.unwrap().to_string());
			idx += s_len.len()+1+_len;
			
		}
	}
	return list;
}



fn main() {
	println!("{}",ben_encode_int(33242));
	println!("{}",ben_decode_int("i3234242e"));
	println!("{}",ben_encode_str("123"));
	println!("{}",ben_decode_str("3:abc"));
	let vec : Vec<String> = vec!["teststring1".to_string(),"63455323".to_string(),"teststring2".to_string(),"8675309".to_string(),"teststring3".to_string(),"1".to_string()];
	println!("{}",ben_encode_list(&vec));
	let parsed = ben_decode_list(ben_encode_list(&vec));
	for i in 0..parsed.len() {
		println!("{0} -> {1}",vec[i],parsed[i]);
	}
	
}

