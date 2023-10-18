//Add it to environment variable: C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\MSBuild\Current\Bin
//Add it to environment variable: C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin

#[link(name = "badmath",kind="static")]
extern "C"{
    fn bad_add(v1:f32,v2:f32)->f32;
}

fn main() {
    println!("Hello, from RUST!");
    let res = unsafe{bad_add(9., 12.)};
    println!("{} ?? are you sure that's right",res);
}
