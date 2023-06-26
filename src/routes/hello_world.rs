use axum::http::{Response, StatusCode};

pub async fn hello_world() -> String {
    String::from("Hello World from a custom file")
}

pub async fn account_delete() -> Response<String> {
    let response_text = String::from("
<!DOCTYPE html>
<html>
<head>
	<title>Account/Data Deletion</title>
</head>
<body>
	<h1>Account/Data Deletion</h1>
    <p> To delete all workout data you can use the Android app and navigate to the Settings, and then Delete All. This will permanently delete all sets stored on the server. </p>
	<p> If you want to delete your account data too please contact us at <a href='mailto:support@verifit.xyz'>support@verifit.xyz</a> </p>
</body>
</html>
");

    let html = format!("<html><body><h1>{}</h1></body></html>", response_text);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html)
        .unwrap()
}

pub async fn privacy_policy() -> Response<String> {
    let response_text = String::from("
<!DOCTYPE html>
<html>
<head>
	<title>Verifit Privacy Policy</title>
</head>
<body>
	<h1>Verifit Privacy Policy</h1>
	<p>Verifit is an Android workout tracker app that allows you to track your workouts both online and offline.</p>
	<p>If you choose to use Verifit's online mode, you will be required to provide your email and password for authentication purposes. Your workout data will be stored on a third-party server.</p>
	<p>We do not sell or share your data with any third-party organizations. Appropriate measures are taken to ensure the security of your data.</p>
	<p>If you have any questions or concerns about our privacy policy, please contact us at <a href='mailto:support@verifit.xyz'>support@verifit.xyz</a>.</p>
	<p>You can also visit our website at <a href='https://github.com/MakisChristou/verifit'>https://github.com/MakisChristou/verifit</a> for more information about our app.</p>
</body>
</html>
");

    let html = format!("<html><body><h1>{}</h1></body></html>", response_text);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html)
        .unwrap()
}
