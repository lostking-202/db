use std::{env, io};
use chrono::{NaiveDateTime, Utc};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct Course{
    pub id:i32,
    pub teacher_id:i32,
    pub name:String,
    pub time:Option<NaiveDateTime>
}
#[actix_rt::main]
async fn main() ->io::Result<()>{
    let a=Course{
        id:1,
        teacher_id:1,
        name:"aaa".to_string(),
        time:Some(Utc::now().naive_utc())
    };
    dotenv().ok();
    let databse_url=env::var("DATABASE_URL").expect("DATABASE_URL 没有在.env文件里设置");
    let db_pool=PgPoolOptions::new().connect(&databse_url).await.unwrap();
    let course_rows=sqlx::query!(r"select id,teacher_id,name,time from course where id=$1",1).fetch_all(&db_pool).await.unwrap();
    let mut courses_list:Vec<Course>=vec![];
    for row in course_rows{
        courses_list.push(Course{
            id:row.id,
            teacher_id:row.teacher_id,
            name:row.name,
            time:Some(chrono::NaiveDateTime::from(row.time.unwrap()))
        })
    };
    println!("Courses={:?}",courses_list);
    Ok(())

}
