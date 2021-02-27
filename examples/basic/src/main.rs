extern crate actix_lua;
extern crate actix;
extern crate futures;
use actix::*;
use actix_lua::{LuaActorBuilder, LuaMessage};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/*  actix-lua /actix 0.9.0
 *
 * https://github.com/geofmureithi/actix-lua
 * 
 */

#[actix_rt::main] 
async fn main () {

    let addr = LuaActorBuilder::new()
        .on_handle_with_lua(r#"return ctx.msg + 42"#)
        .build()
        .unwrap()
        .start();

    let res = addr.send(LuaMessage::from(100)).await;
    
    
    print_type_of(&res);
    match res.unwrap() {
            LuaMessage::String(s) => println!("String:{}",s),
            LuaMessage::Integer(s) => println!("Integer:{}",s),
            LuaMessage::Number(s) => println!("Number:{}",s),
            LuaMessage::Boolean(s) => println!("Boolean:{}",s),
            // ignore everything else
            _ => println!("unknown"),

    }


}
