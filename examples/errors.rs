extern crate lua_patterns2;
use lua_patterns2::errors::PatternError;

fn main() {
   let bad = [
            ( "bonzo %",              PatternError::MalformedPattern("ends with '%'") ),
            ( "bonzo (dog%(",         PatternError::UnfinishedCapture                 ),
            ( "alles [%a%[",          PatternError::MalformedPattern("missing ']'")   ),
            ( "bonzo (dog (cat)",     PatternError::UnfinishedCapture                 ),
            ( "frodo %f[%A",          PatternError::MalformedPattern("missing ']'")   ),
            ( "frodo (1) (2(3)%2)%1", PatternError::InvalidCaptureIndex(Some(1))      ),
    ];

    for p in bad.iter() {
        let res = lua_patterns2::LuaPattern::new_try(p.0);
        if let Err(e) = res {
            assert_eq!(e, p.1);
        } else {
            println!("'{}' was fine",p.0);
        }
   }

}
