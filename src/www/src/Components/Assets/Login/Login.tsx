import React, { FormEvent, useState } from "react";
import "./Login.css";
import "./db"

function Login({ isLoggedIn, toggleLoggedIn }: { isLoggedIn: boolean, toggleLoggedIn: () => void }) {
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const submit = (e: FormEvent<HTMLInputElement>) => {
		e.preventDefault();
		console.log(e);
	}
	return (
	<div className="form-container">
		<form className="login-form">
			<label htmlFor="loginUname">Username:</label>
			<input type="text" id="loginUname" value={username} onInput={(e) => setUsername(e.currentTarget.value)} title="Enter your user name" />
			<label htmlFor="loginPasswd">Password:</label>
			<input type="password" id="loginPasswd" value={password} onInput={(e) => setPassword(e.currentTarget.value)} title="Enter your password" />
			<input type="submit" id="loginSubmit" value="Log In" onSubmit={submit} />
		</form>
	</div>
	)
}

export default Login;
