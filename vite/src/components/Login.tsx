import { useState } from "react";
import {
    AuthState,
    useAuthContextActions,
    useAuthContextData,
} from "../context/AuthContext";

function Login() {
    const [isSignUp, setSignUp] = useState(false);
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [passwordMatch, setPasswordMatch] = useState(true);
    const [confirmedPassword, setConfirmedPassword] = useState("");
    const auth = useAuthContextData();
    const { setAuthenticated } = useAuthContextActions();

    const signUp = () => {
        if (password !== confirmedPassword) {
            setPasswordMatch(false);
            return;
        }
        setAuthenticated();
    };

    return (
        <dialog
            open={auth === AuthState.AuthInProgress}
            className="p-4 rounded-xl bg-green-secondary"
        >
            <div className="flex-col min-h-96 w-96">
                <div>
                    <form method="dialog">
                        <div className="flex-col">
                            <div className="pb-4">
                                <input
                                    placeholder="Email"
                                    type="text"
                                    name="email"
                                    className="rounded px-2 py-1"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                />
                            </div>
                            <div className="pb-4">
                                <input
                                    placeholder="Password"
                                    type="password"
                                    name="password"
                                    className="rounded px-2 py-1"
                                    value={password}
                                    onChange={(e) =>
                                        setPassword(e.target.value)
                                    }
                                />
                            </div>

                            <div className="pb-4">
                                {isSignUp ? (
                                    <input
                                        placeholder="Confirm Password"
                                        type="password"
                                        name="confirmedPassword"
                                        className={`rounded px-2 py-1`}
                                        value={confirmedPassword}
                                        onChange={(e) =>
                                            setConfirmedPassword(e.target.value)
                                        }
                                    />
                                ) : (
                                    <div className="h-8"></div>
                                )}
                            </div>

                            <div className="pb-4">
                                {passwordMatch ? (
                                    <div className="h-6"></div>
                                ) : (
                                    <span className="font-small text-orange-primary">
                                        {"Passwords must match"}
                                    </span>
                                )}
                            </div>

                            <div className="self-justify-center pb-4">
                                <button
                                    type="button"
                                    onClick={() => {
                                        console.log(
                                            `${email}:${password}:${confirmedPassword}`
                                        );
                                        signUp();
                                    }}
                                    className="h-12 w-full rounded-lg bg-yellow-primary hover:bg-orange-primary"
                                >
                                    <span className="inline-block align-middle text-green-primary">
                                        {isSignUp
                                            ? "Create account"
                                            : "Sign in"}
                                    </span>
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
                <div className="self-justify-center">
                    <button
                        onClick={() => setSignUp(!isSignUp)}
                        className="self-center"
                    >
                        <span className="inline-block align-middle text-white hover:text-yellow-primary">
                            {isSignUp
                                ? "Already have an account?"
                                : "Create an account"}
                        </span>
                    </button>
                </div>
            </div>
        </dialog>
    );
}

export default Login;
