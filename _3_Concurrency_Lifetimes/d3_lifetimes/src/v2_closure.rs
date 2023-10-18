#[derive(Debug)]
pub struct Hider {
    pub public: String,
    hidden: String,
    hidden_accessed:i32,
}

impl Hider{
    pub fn new(public: String,hidden:String) -> Hider {
        Hider { public, hidden, hidden_accessed:0 }
    }

    pub fn edit<F>(&mut self,f: F) where F:FnOnce(&mut String){ //Fn is generic version, FnOnce is more specific.
        f(&mut self.hidden);
        self.hidden_accessed += 1;
    }
}

#[derive(Debug)]
pub struct HideVec{
    v:Vec<String>
}

impl HideVec {
    pub fn new(n:usize) -> Self{
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(String::new())
        }
        HideVec { v }
    }

    //We do couting in function, so we make sure that we can change function ourselves, so function must be mutable
    //Also FnMut can be copied, so the for loop for f is not a problem. What if we use FnOnce, it gives error, couldnot be copied and says f is consumed before.
    pub fn edit_all<F>(&mut self,mut f:F) where F:FnMut(&mut String){//FnMut:Changeable function, function itself changes over time.
        for item in &mut self.v{
            f(item)
        }
    }
}


fn main(){
    let mut h = Hider::new("showme".to_string(),"hideme".to_string());
    h.edit(|s|s.push_str(" please"));

    println!("Hider = {:?}",h);

    let mut hv = HideVec::new(6);
    let mut count = 0;
    hv.edit_all(|s| {
        s.push_str(&format!("Item = {}",count));
        count +=1;
    });

    println!("HV = {:?}, count = {}",hv,count);
}
