mod auth;
mod user;
mod category;
mod comment;
mod post;


use actix_web::web;

use self::auth::{
    get_user,
    login_user_handler,
    register_user_handler
};

use self::user::{
    create_user,
    find_user_by_email,
    update_user,
    delete_user
};

use self::category::{
    get_categories,
    get_category,
    create_category,
    update_category,
    delete_category
};

use self::post::{
    get_posts,
    get_post_relation,
    get_post,
    create_post,
    update_post,
    delete_post
};

use self::comment::{
    get_comments,
    get_comment,
    create_comment,
    update_comment,
    delete_comment
};

pub fn router_config(conf: &mut web::ServiceConfig) {
    let router = web::scope("/api")
  
        .service(register_user_handler)
        .service(login_user_handler)
        .service(get_user)
        .service(get_categories)
        .service(get_category)
        .service(create_category)
        .service(update_category)
        .service(delete_category)
        .service(get_posts)
        .service(get_post_relation)
        .service(get_post)
        .service(create_post)
        .service(update_post)
        .service(delete_post)
        .service(get_comments)
        .service(get_comment)
        .service(create_comment)
        .service(update_comment)
        .service(delete_comment)
        .service(create_user)
        .service(find_user_by_email)
        .service(update_user)
        .service(delete_user);

    conf.service(router);
}