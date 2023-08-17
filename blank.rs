/*
//NEW
pub async fn login (
    State(database): State<Store>,
    Form(creds): Form<User>, //The credentials will be sent back on submit of the html form
) -> Result<Response<String>, AppError> {
    let mut context = Context::new();
    //will store the context info if any errors related to login occur
    let mut error = LoginErrors {
        missing_cred: false,
        missing_cred_message: "".to_string(),
        invalid_pass: false,
        invalid_pass_message: "".to_string(),
        no_user: false,
        no_user_message: "".to_string(),
    };

    //basic template with an empty error context, will use if no errors found
    let mut template_name = {
        context.insert("error", &error);
        "index.html"
    };

    let error_flag = false;

    //Missing Credentials ERROR situation
    if creds.email.is_empty() || creds.password.is_empty() {
        error.missing_cred = true;
        error.missing_cred_message = "Please enter your username and password to login".to_string();
        template_name = {
            context.insert("error", &error);
            "index.html"
        };

        error_flag = true;
    }

    let existing_user = database.get_user(&creds.email).await?;

    //INVALID USER ERROR situation
    if existing_user.is_err() {
        //ERROR NO USER FOUND
        error.no_user = true;
        error.no_user_message = "We can't find that username in our records, please try again.".to_string();
        template_name = {
            context.insert("error", &error);
            "index.html"
        };
        error_flag = true;
    }

    //Only go forward with the user IF we haven't erred
    if !error_flag {
        //VALID USER
        //use argon2 to reverse hash and verify the given password
        let is_password_correct =
            match argon2::verify_encoded(&existing_user.password, creds.password.as_bytes()) {
                Ok(result) => result,
                Err(_) => {
                    return Err(AppError::InternalServerError);
                }
            };

        //INVALID PASSWORD ERROR situation, we need to render the template with that error
        if !is_password_correct {
            error.invalid_pass = true;
            error.invalid_pass_message =
                "The password you entered was not correct, please try again.".to_string();
            template_name = {
                context.insert("error", &error);
                "index.html"
            };

            error_flag = true;
        }

        if !error_flag {
            //Otherwise the user AND password is correct, so we want to set the JWT
            //Now we have validated the users creds, lets make claims piece of the token
            let claims = Claims {
                id: 0,
                email: creds.email.to_owned(),
                exp: get_timestamp_after_8_hours(),
            };

            //Build the JWT token from it's parts: Header, Payload(claims), and the Encoding keys built from the Secret
            //Thanks to jsonwebtoken library, we can easily encode if we have these three things
            //the header we use will just be set to default for now
            let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding) //here is where we use the KEYS we build in user.rs
                .map_err(|_| AppError::MissingCredentials)?;

            //we will store the token as a cookie
            let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

            let rendered = TEMPLATES
                .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                .unwrap_or_else(|err| {
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let mut response = Response::builder()
                .status(StatusCode::FOUND)
                .body(rendered) //stick the html that gets rendered, inside the body here!
                .unwrap();

            response
                .headers_mut()
                .insert(LOCATION, HeaderValue::from_static("/"));
            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string()).unwrap(),
            );
            Ok(response) //return with NO ERROR, will only get this with !error_flag all the way through
        }

    }

    //Otherwisee Errored by either having a blank form, or an invalid username
    //Take what templates we made for the errors, and render them with the response (no token1)
    let rendered = TEMPLATES
        .render(template_name, &context) //render takes all the context attached in template_name and inserts it
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .body(rendered) //stick the html that gets rendered, inside the body here!
        .unwrap();

    Ok(response) //return on any error

}


pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>, //The credentials will be sent back on submit of the html form
) -> Result<Response<String>, AppError> {
    let mut context = Context::new();
    //will store the context info if any errors related to login occur
    let mut error = LoginErrors {
        missing_cred: false,
        missing_cred_message: "".to_string(),
        invalid_pass: false,
        invalid_pass_message: "".to_string(),
        no_user: false,
        no_user_message: "".to_string(),
    };

    //basic template with an empty error context
    let mut template_name = {
        context.insert("error", &error);
        "index.html"
    };

    //Missing Credentials ERROR situation
    if creds.email.is_empty() || creds.password.is_empty() {
        error.missing_cred = true;
        error.missing_cred_message = "Please enter your username and password to login".to_string();
        template_name = {
            context.insert("error", &error);
            "index.html"
        };

        let rendered = TEMPLATES
            .render(template_name, &context) //render takes all the context attached in template_name and inserts it
            .unwrap_or_else(|err| {
                error!("Template rendering error: {}", err);
                panic!()
            });

        let response = Response::builder()
            .status(StatusCode::FOUND)
            .body(rendered) //stick the html that gets rendered, inside the body here!
            .unwrap();

        Ok(response) //return right away on error
    } else {
        let existing_user = database.get_user(&creds.email).await?;
        //TODO why is this breaking here? nesting?
        if existing_user.is_ok() {

            //use argon2 to reverse hash and verify the given password
            let is_password_correct =
                match argon2::verify_encoded(&existing_user.password, creds.password.as_bytes()) {
                    Ok(result) => result,
                    Err(_) => {
                        return Err(AppError::InternalServerError);
                    }
                };

            //INVALID PASSWORD ERROR situation, we need to render the template with that error
            if !is_password_correct {
                error.invalid_pass = true;
                error.invalid_pass_message =
                    "The password you entered was not correct, please try again.".to_string();
                template_name = {
                    context.insert("error", &error);
                    "index.html"
                };

                let rendered = TEMPLATES
                    .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                    .unwrap_or_else(|err| {
                        error!("Template rendering error: {}", err);
                        panic!()
                    });

                let response = Response::builder()
                    .status(StatusCode::FOUND)
                    .body(rendered) //stick the html that gets rendered, inside the body here!
                    .unwrap();

                Ok(response) //return right away on error
            } else {
                //the password is CORRECT, and we can create the JWT token

                //Now we have validated the users creds, lets make claims piece of the token
                let claims = Claims {
                    id: 0,
                    email: creds.email.to_owned(),
                    exp: get_timestamp_after_8_hours(),
                };

                //Build the JWT token from it's parts: Header, Payload(claims), and the Encoding keys built from the Secret
                //Thanks to jsonwebtoken library, we can easily encode if we have these three things
                //the header we use will just be set to default for now
                let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding) //here is where we use the KEYS we build in user.rs
                    .map_err(|_| AppError::MissingCredentials)?;

                //we will store the token as a cookie
                let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

                let rendered = TEMPLATES
                    .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                    .unwrap_or_else(|err| {
                        error!("Template rendering error: {}", err);
                        panic!()
                    });

                let mut response = Response::builder()
                    .status(StatusCode::FOUND)
                    .body(rendered) //stick the html that gets rendered, inside the body here!
                    .unwrap();

                response
                    .headers_mut()
                    .insert(LOCATION, HeaderValue::from_static("/"));
                response.headers_mut().insert(
                    SET_COOKIE,
                    HeaderValue::from_str(&cookie.to_string()).unwrap(),
                );

                Ok(response) //return with NO ERROR
            }
        } else {
            //ERROR NO USER FOUND
            error.no_user = true;
            error.no_user_message = "We can't find that username in our records, please try again.".to_string();
            template_name = {
                context.insert("error", &error);
                "index.html"
            };

            let rendered = TEMPLATES
                .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                .unwrap_or_else(|err| {
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let response = Response::builder()
                .status(StatusCode::FOUND)
                .body(rendered) //stick the html that gets rendered, inside the body here!
                .unwrap();

            Ok(response) //return right away on error
        }
    }
}


*/