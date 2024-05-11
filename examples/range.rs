extern crate lua_patterns2;

use lua_patterns2::LuaPattern;
#[cfg(feature = "std")]
use lua_patterns2::LuaPatternBuilder;

fn main() {
	let mut m = LuaPattern::new("(%a+) one");
	let text = " hello one two";
	assert!(m.matches(text));
	assert_eq!(m.capture(1),1..6);
	assert_eq!(m.capture(0),1..10);

	// if cfg!(feature = "std") {
	//     let v = m.captures(text);
	//     assert_eq!(v, &["hello one","hello"]);
	// }
	// if cfg!(feature = "heapless") {
	//     let v = m.captures_heapless(text);
	//     assert_eq!(v, &["hello one","hello"]);
	// }


	// if cfg!(feature = "std") {
	//     let mut v = Vec::new();
	//     assert!(m.capture_into(text,&mut v));
	//     assert_eq!(v, &["hello one","hello"]);
	// }
	// if cfg!(feature = "heapless") {
	//     let mut v = heapless::Vec::new();
	//     assert!(m.capture_into_heapless(text,&mut v).unwrap());
	//     assert_eq!(v, &["hello one","hello"]);
	// }
	
	// if cfg!(feature = "std") {
	//     let bytes = &[0xFF,0xEE,0x0,0xDE,0x24,0x24,0xBE,0x0,0x0];      

	//     let patt = LuaPatternBuilder::new()
	//         .bytes_as_hex("DE24")
	//         .text("+")
	//         .bytes(&[0xBE])
	//         .build();
		
	//     let mut m = LuaPattern::from_bytes(&patt);
	//     assert!(m.matches_bytes(bytes));
	//     assert_eq!(&bytes[m.capture(0)], &[0xDE,0x24,0x24,0xBE]);
	
	
	//     let mut m = LuaPattern::new("(%S+)%s*=%s*(%S+);%s*");
	//     let res = m.gsub("a=2; b=3; c = 4;","'%2':%1 ");
	//     println!("{}",res);
		
	//     let mut m = LuaPattern::new("%s+");
	//     let res = m.gsub("hello dolly you're so fine","");
	//     println!("{}",res);
	// }
}
