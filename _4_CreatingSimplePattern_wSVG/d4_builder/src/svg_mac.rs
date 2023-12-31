//Declarative Macros
//anything we want to repeat and anything we want to name is given with $ sign.

macro_rules! svg{
    ($s:ident $($p:ident = $v:expr),* => $($c:tt)*) => {
        SvgTag::new(stringify!($s))$(.$p($v))*$(.child(svg!$c))* //.property($p,$v)
    };
    ($s:ident $($p:ident = $v:expr),*) => {
        SvgTag::new(stringify!($s))$(.$p($v))* //.property($p,$v)
    };
}

#[cfg(test)]
pub mod macro_test{
    use super::*;
    use crate::SvgTag;

     #[test]
     pub fn test_make_svg_macro(){
        let csvg = svg!{
            svg => {rect x=7,y=7,w=9,h=9}
        };
        
        let dsvg = SvgTag::new("svg").child(SvgTag::new("rect").x(7).y(7).w(9).h(9));

        assert_eq!(csvg,dsvg);
     }
}