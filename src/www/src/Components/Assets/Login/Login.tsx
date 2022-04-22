import React, { useState } from "react";
import "./Login.css";

function Login() {
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	return (
	<div className="form-container">
		<form className="login-form">
			<label htmlFor="loginUname">Username:</label>
			<input type="text" id="loginUname" value={username} onInput={(e) => setUsername(e.currentTarget.value)} title="Enter your user name" />
			<label htmlFor="loginPasswd">Password:</label>
			<input type="password" id="loginPasswd" value={password} onInput={(e) => setPassword(e.currentTarget.value)} title="Enter your password" />
			<input type="submit" id="loginSubmit" value="Log In" />
		</form>
	</div>
	)
}

export default Login;
