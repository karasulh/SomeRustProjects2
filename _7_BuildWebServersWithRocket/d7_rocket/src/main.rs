//rustup override set nightly

#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket::response::{Responder,NamedFile};
use rocket::request::Form;

use std::path::PathBuf;
pub mod error;
use crate::error::DoodleWebErr;

#[macro_use] extern crate rocket_contrib;

use diesel::prelude::*;
use d6_doodle::{schema::*,models::*};

pub mod session;
use crate::session::Session;

use rocket::State;
use rocket::http::{Cookie,Cookies};

use maud::{html,DOCTYPE};

#[get("/")]
fn root()->Result<impl Responder<'static>,failure::Error>{
    NamedFile::open("site/static/index.html").map_err(|e|e.into())
}

#[get("/<path..>")]
fn static_file(path:PathBuf)->Result<impl Responder<'static>,DoodleWebErr>{
    let path = PathBuf::from("site/static").join(path);
    NamedFile::open(path).map_err(|e|e.into())
}

#[derive(FromForm)]
pub struct LoginData{
    name:String,
    pass:String
}

#[database("doodlebase")]
pub struct DPool(diesel::pg::PgConnection);

#[post("/login",data="<dt>")]
fn login(dt:Form<LoginData>,db:DPool,ss:State<session::Session>,mut cookies: Cookies)->Result<impl Responder<'static>,DoodleWebErr>{
    let ldform = dt.into_inner();
    let vals = users::table.filter(users::name.eq(ldform.name)).load::<User>(&db.0)?;
    let user = vals.iter().next().ok_or(DoodleWebErr::UserDoesNotExistError)?;

    if !user.verify_pass(&ldform.pass){
        return Err(DoodleWebErr::PasswordError);
    }

    let sess_id = ss.put(user.clone());
    cookies.add(Cookie::new("login",sess_id.to_string()));

    //Ok("Password passed")
    Ok(
        html!{
            (DOCTYPE)
            head{
                meta charset = "utf-8";
            }
            body{
                h1{"Welcome" (user.name)}
                h2{"Ask a question"}
                div style="border:1px solid black;"{
                    form action = "question" method="POST"{
                        "Question" input type="text" name="question";
                        "Options" input type="text" name="options";
                        input type="submit" value="Ask Question";
                    }
                }
            }
        }
    )
}

 #[derive(FromForm)]
 pub struct QuestionForm{
    question:String,
    options:String,
 }

 #[post("/question", data="<dt>")]
 pub fn ask_question(dt:Form<QuestionForm>,st:State<session::Session>,cookies:Cookies,db:DPool)->
    Result<impl Responder<'static>,DoodleWebErr>{

        let login = cookies.get("login").ok_or(DoodleWebErr::NoCookie)?.value();
        let user = st.get(login.parse().map_err(|_|DoodleWebErr::NoCookie)?).ok_or(DoodleWebErr::NoSession)?;
        
        let poll = NewPoll{
            question:&dt.question,
            options:&dt.options,
            owner:Some(user.id),
        };

        let p_added:Poll = diesel::insert_into(polls::table).values(&poll).get_result(&db.0)?;

        Ok(
            html!{
                (DOCTYPE)
                html{
                    head{
                        meta charset = "utf-8";
                    }
                    body{
                        h1{"Interesting Question"}
                        (p_added.question) "??" br;
                        (p_added.options)
                    }
                }
            }
        )
    }

fn main() {

    let sess = session::Session::new();

    rocket::ignite().mount("/",routes![root,static_file,login,ask_question])
        .attach(DPool::fairing())
        .manage(sess)
        .launch();
    println!("Hello, world!");
}
